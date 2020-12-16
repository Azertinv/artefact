extends MarginContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)

export(bool) var max_speed: bool = true

export(String, MULTILINE) var tooltip = "RegisterViewer"

onready var run_pause_btn = $HBoxContainer/RunPause
onready var step_btn = $HBoxContainer/Step

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
			print(inst_per_frame)
		artefact.run(inst_per_frame)

func _on_RunPause_toggled(button_pressed):
	if button_pressed:
		state = STATE_RUNNING
		run_pause_btn.text = "Pause"
		step_btn.disabled = true
	else:
		state = STATE_PAUSED
		run_pause_btn.text = "Run"
		step_btn.disabled = false

func _on_Step_pressed():
	if state == STATE_PAUSED:
		artefact.run(1)

func _on_ExecutionController_mouse_entered():
	TooltipManager.tooltip = tooltip
