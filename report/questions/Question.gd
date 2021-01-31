extends HBoxContainer

export(String, MULTILINE) var answer = ""
export(String) var save_section = "CHANGEME"

func get_answer_helper(answer_box) -> String:
	var result = ""
	for c in answer_box.get_children():
		if c is Label:
			result += c.text + " "
		elif c is LineEdit:
			result += c.get_number() + " "
		elif c is HBoxContainer: # not an AnswerBox
			result += get_answer_helper(c)
		elif c is CanvasLayer:
			pass
		else:
			result += get_answer_helper(c)
	return result

func get_answer() -> String:
	return get_answer_helper($Answer).strip_edges()

func is_good_answer() -> bool:
	return answer == get_answer()

func mark_question_as_answered() -> void:
	$CheckBox.pressed = true
	$Answer.queue_free()
	var new_answer = Label.new()
	new_answer.text = answer
	add_child(new_answer)

func _ready() -> void:
	if answer == "":
		push_error("Question "+self.name+" has empty answer")
		breakpoint
	if Save.has_section_key(save_section, name):
		var save_data = Save.get_value(save_section, name)
		if save_data["answered"]:
			mark_question_as_answered()
		elif save_data["response"]:
			$Answer/AnswerBox.add_child(save_data["response"].instance())
	else:
		_exit_tree()

func set_owners_on_answers(new_owner: Node, node: Node):
	if node is Answer:
		if new_owner != node:
			node.owner = new_owner
		node.update_saved_properties()
	for c in node.get_children():
		set_owners_on_answers(new_owner, c)

func _exit_tree():
	var save_data = {}
	if $CheckBox.pressed:
		save_data["answered"] = true
	else:
		save_data["answered"] = false
		if $Answer/AnswerBox.get_child_count() > 0:
			var to_save = $Answer/AnswerBox.get_child(0)
			set_owners_on_answers(to_save, to_save)
			var packed_scene = PackedScene.new()
			packed_scene.pack(to_save)
			save_data["response"] = packed_scene
		else:
			save_data["response"] = null
	Save.set_value(save_section, name, save_data)
