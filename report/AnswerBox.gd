extends PanelContainer

export(Array, String) var possible_answers = []

var answer_prompt_scene = preload("res://report/AnswerPrompt.tscn")

func prompt_for_new_answer() -> void:
	var answer_prompt = answer_prompt_scene.instance()
	answer_prompt.destination = self
	answer_prompt.possible_answers = possible_answers
	add_child(answer_prompt)

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed and event.button_index == BUTTON_LEFT:
			accept_event()
			prompt_for_new_answer()
