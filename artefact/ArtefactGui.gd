extends MarginContainer

onready var register_viewer = $HBoxContainer/RightPanel/RegisterViewer

func _ready():
	PauseScreen.can_pause = true
	Indicator.indicate(register_viewer)
	yield(register_viewer, "mouse_entered")
	Indicator.clear()
