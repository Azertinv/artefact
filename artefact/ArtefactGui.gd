extends MarginContainer

func _process(_delta):
	if Input.is_action_just_pressed("ui_accept"):
		$Artefact.run_one()
