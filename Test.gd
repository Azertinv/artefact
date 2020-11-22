extends Node

func _process(_delta):
	if Input.is_action_just_pressed("ui_accept"):
		$HBoxContainer/ByteEdit.set_trits(
			[0,1,1,1,1,1,1,1,-1]
		)
		$HBoxContainer/WordEdit.set_trits(
			[0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,-1]
		)
