extends MarginContainer

var max_i: int = 1000

enum {
	STATE_RUNNING,
	STATE_PAUSED,
}

var state = STATE_PAUSED

func _process(_delta):
	if state == STATE_RUNNING:
		if Input.is_action_just_pressed("dbg_start_pause"):
			state = STATE_PAUSED
		else:
#			if delta > 1.0 / 60.0 * 1.01:
#				max_i -= 1 + max_i/100
#			else:
#				max_i += 1 + max_i/100
#			print(max_i*60)
			$Artefact.run(max_i)
	elif state == STATE_PAUSED:
		if Input.is_action_just_pressed("dbg_start_pause"):
			state = STATE_RUNNING
		elif Input.is_action_just_pressed("dbg_step"):
			$Artefact.run(1)
