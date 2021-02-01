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
	Helper.init_format($Answer, data["format"], data["possibilities"])

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

#TODO save system
