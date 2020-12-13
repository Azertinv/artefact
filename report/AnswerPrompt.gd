extends CanvasLayer

var destination = null
var possible_answers_scenes = []

onready var answer_list = $CenterContainer/VBoxContainer

func _ready() -> void:
	var current_focus_control = answer_list.get_focus_owner()
	if current_focus_control:
		current_focus_control.release_focus()
	var first = true
	for s in possible_answers_scenes:
		var answer = s.instance()
		answer.interactable = false
		answer.connect("pressed", self, "_on_Answer_pressed")
		if not first:
			answer_list.add_child(HSeparator.new())
		else:
			first = false
		answer_list.add_child(answer)

func _input(event: InputEvent):
	if event.is_action_pressed("ui_cancel"):
		get_tree().set_input_as_handled()
		queue_free()

func _on_Answer_pressed(answer) -> void:
#	print("AnswerPrompt: " + str(answer))
	if answer.get_parent():
		answer.get_parent().remove_child(answer)
	answer.interactable = true
	while destination.get_child_count() > 0:
		destination.remove_child(destination.get_child(0))
	destination.add_child(answer)
	queue_free()
