extends PanelContainer

export(Array, String) var possible_answers = []

var answer_prompt_scene = preload("res://report/AnswerPrompt.tscn")

func _ready() -> void:
	if possible_answers.size() == 0:
		push_error("AnswerBox: possible answers cannot be empty")
		breakpoint

func prompt_for_new_answer() -> void:
	var answer_prompt = answer_prompt_scene.instance()
	answer_prompt.destination = self
	answer_prompt.possible_answers = possible_answers
	get_tree().get_root().add_child(answer_prompt)

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed and event.button_index == BUTTON_LEFT:
			accept_event()
			prompt_for_new_answer()
