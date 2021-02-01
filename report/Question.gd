extends HBoxContainer

export(String) var question_id = ""

var answer: String
var answer_format: String

func _ready():
	if question_id == "":
		push_error("NewQuestion spawned with no id")
		return
	var data = ReportLoader.questions[question_id]
	answer_format = data["answer"]
	answer = answer_format.replace("%a", "")
	answer = answer.replace("%t", "")
	if Save.has_section_key("Questions", question_id):
		var save_data = Save.get_value("Questions", question_id)
		if save_data["answered"]:
			mark_question_as_answered()
		elif save_data["response"]:
			$Answer.replace_by(save_data["response"].instance())
	else:
		Helper.init_format($Answer, data["format"], data["possibilities"])
		_exit_tree()

func set_owners_on_answers(new_owner: Node, node: Node):
	if new_owner != node:
		node.owner = new_owner
	for c in node.get_children():
		set_owners_on_answers(new_owner, c)

func _exit_tree():
	var save_data = {}
	if $CheckBox.pressed:
		save_data["answered"] = true
	else:
		save_data["answered"] = false
		set_owners_on_answers($Answer, $Answer)
		var packed_scene = PackedScene.new()
		packed_scene.pack($Answer)
		save_data["response"] = packed_scene
	Save.set_value("Questions", question_id, save_data)

func get_answer_helper(answer_box) -> String:
	var result = ""
	for c in answer_box.get_children():
		if c is Label:
			result += c.text + " "
		elif c is LineEdit:
			result += c.get_number() + " "
		elif c is CanvasLayer:
			pass
		else: # go down one layer
			result += get_answer_helper(c)
	return result

func get_answer() -> String:
	return get_answer_helper($Answer).strip_edges()

func is_good_answer() -> bool:
	return answer == get_answer()

func mark_question_as_answered() -> void:
	$CheckBox.pressed = true
	$Answer.queue_free()
	Helper.init_format(self, answer_format, [])
