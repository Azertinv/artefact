extends HBoxContainer

export(String, MULTILINE) var answer = ""

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

