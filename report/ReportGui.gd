extends VBoxContainer

signal completed

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
	if good_answers.size() >= 3:
		for a in good_answers:
			a.mark_question_as_answered()
	if completed:
		$CheckAnswerTimer.stop()
		emit_signal("completed")

func _input(event: InputEvent):
	if OS.is_debug_build() and event.is_action_pressed("cheat"):
		for q in get_children():
			if not q.is_in_group("questions"):
				continue
			if not q.get_node("CheckBox").pressed:
				q.mark_question_as_answered()

func _ready():
	if !Save.has_section_key("ReportGui", "TutorialDone"):
		report_tutorial()

func report_tutorial():
	var answer_box = $WhatIsEqual/Answer/AnswerBox
	Indicator.indicate(answer_box)
	$Dialog.play_dialog("ReportGui/Tutorial", false)
	yield($Dialog, "dialog_completed")
	yield(answer_box, "mouse_entered")
	Indicator.clear()
	Save.set_value("ReportGui", "TutorialDone", true)
