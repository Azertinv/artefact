extends HBoxContainer

export(String, MULTILINE) var answer = ""
export(String) var save_section = "CHANGEME"

func get_answer(answer_box) -> String:
	var result = ""
	for c in answer_box.get_children():
		if c is Label:
			result += c.text
		elif c is LineEdit:
			result += " " + c.get_number()
		elif c is HBoxContainer: # not an AnswerBox
			result += get_answer(c)
		elif c is CanvasLayer:
			pass
		else:
			result += " " + get_answer(c)
	return result

func is_good_answer() -> bool:
	var response = get_answer($Answer)
#	print(response)
	return answer == response

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
			# already setup variable from _ready will be saved with the node
			$Answer/AnswerBox.add_child(Helper.unpack_node_tree(save_data["response"]))
	else:
		_exit_tree()

func _exit_tree():
	var save_data = {}
	if $CheckBox.pressed:
		save_data["answered"] = true
	else:
		save_data["answered"] = false
		if $Answer/AnswerBox.get_child_count() > 0:
			save_data["response"] = Helper.pack_node_tree($Answer/AnswerBox.get_child(0))
		else:
			save_data["response"] = null
	Save.set_value(save_section, name, save_data)
