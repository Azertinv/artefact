extends Label

func _process(_delta):
	text = ""
	if Input.is_action_pressed("trit_zero"):
		text = "0"
	if Input.is_action_pressed("trit_one"):
		text = "1"
	if Input.is_action_pressed("trit_tern"):
		text = "T"
