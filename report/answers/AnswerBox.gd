extends PanelContainer

export(Array, String) var possible_answers = []
var possible_answers_scenes = []

var answer_prompt_scene = preload("res://report/AnswerPrompt.tscn")

func _ready() -> void:
	if possible_answers.size() == 0:
		push_error("AnswerBox: possible answers cannot be empty")
	for a in possible_answers:
		var scene_path = "res://report/answers/"+a+".tscn"
		possible_answers_scenes.append(load(scene_path))

func prompt_for_new_answer() -> void:
#	print(self.name+" displaying AnswerPrompt")
	var answer_prompt = answer_prompt_scene.instance()
	answer_prompt.destination = self
	answer_prompt.possible_answers_scenes = possible_answers_scenes
	get_tree().get_root().add_child(answer_prompt)

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			accept_event()
			prompt_for_new_answer()
