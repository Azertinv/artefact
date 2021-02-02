extends VBoxContainer

export(int, 0, 100) var min_question_to_check = 3
export(String, FILE, "*.json") var filepath

signal completed

var question_scene = preload("res://report/Question.tscn")

var questions: Dictionary
var answers: Dictionary

func _ready():
	if filepath == "":
		push_error("ReportGui loaded with no data")
		return
	var file = File.new()
	file.open(filepath, File.READ)
	var content = JSON.parse(file.get_as_text()).result
	file.close()
	questions = content["questions"]
	answers = content["answers"]
	for q in questions:
		var question = question_scene.instance()
		question.question_id = q
		add_child(question)

func _on_CheckAnswerTimer_timeout() -> void:
	var completed = true
	var good_answers: Array = []
	for q in get_children():
		if not q.is_in_group("questions"):
			continue
		if q.get_node("CheckBox").pressed:
			continue
		completed = false
		if q.is_good_answer():
			good_answers.append(q)
	if good_answers.size() >= min_question_to_check:
		for a in good_answers:
			a.mark_question_as_answered()
	if completed:
		$CheckAnswerTimer.stop()
		emit_signal("completed")

func _input(event: InputEvent):
	if OS.is_debug_build() and event.is_action_pressed("cheat"):
		get_tree().set_input_as_handled()
		for q in get_children():
			if not q.is_in_group("questions"):
				continue
			if not q.get_node("CheckBox").pressed:
				print(q.name + ": " + q.get_answer())
				q.mark_question_as_answered()
