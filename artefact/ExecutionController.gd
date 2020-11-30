extends HBoxContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)

export(bool) var max_speed: bool = true

enum {
	STATE_RUNNING,
	STATE_PAUSED,
}

var state = STATE_PAUSED
var inst_per_frame: int = 1009

func _process(delta):
	if state == STATE_RUNNING:
		if max_speed:
			if delta > 1.0 / 60.0 * 1.01:
				inst_per_frame -= 1 + inst_per_frame/100
			else:
				inst_per_frame += 1 + inst_per_frame/100
			print(inst_per_frame * 60)
		artefact.run(inst_per_frame)

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
