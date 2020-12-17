extends MarginContainer

func _ready():
#	yield(get_tree().create_timer(3), "timeout")
	DialogScene.play_dialog("ArithmeticArtefact/Tutorial")
