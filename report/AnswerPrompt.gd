extends CanvasLayer

var destination = null
var possible_answers_scenes = []

func _ready():
	for s in possible_answers_scenes:
		var answer = s.instance()
		answer.interactable = false
		answer.connect("pressed", self, "_on_Answer_pressed")
		$CenterContainer/VBoxContainer.add_child(answer)

func _on_Answer_pressed(answer):
	print("AnswerPrompt: " + str(answer))
	$CenterContainer/VBoxContainer.remove_child(answer)
	answer.interactable = true
	while destination.get_child_count() > 0:
		destination.remove_child(destination.get_child(0))
	destination.add_child(answer)
	queue_free()
