extends VBoxContainer

func _on_CheckAnswerTimer_timeout() -> void:
	var good_answers = []
	for q in get_children():
		if not q.is_in_group("questions"):
			continue
		if q.get_node("CheckBox").pressed:
			continue
		if q.is_good_answer():
			good_answers.append(q)
	if good_answers.size() >= 3:
		for a in good_answers:
			a.mark_question_as_answered()
			a.remove_from_group("questions")
