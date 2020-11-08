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

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("ui_accept"):
		if $CheckBox.pressed:
			return
		if is_good_answer():
			mark_question_as_answered()
