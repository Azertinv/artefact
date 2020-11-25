extends HBoxContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)

var max_i: int = 1000

enum {
	STATE_RUNNING,
	STATE_PAUSED,
}

var state = STATE_PAUSED

func _process(_delta):
	if state == STATE_RUNNING:
		artefact.run(max_i)

func _on_RunPause_toggled(button_pressed):
	if button_pressed:
		state = STATE_RUNNING
		$RunPause.text = "Pause"
	else:
		state = STATE_PAUSED
		$RunPause.text = "Run"

func _on_Step_pressed():
	if state == STATE_PAUSED:
		artefact.run(1)
