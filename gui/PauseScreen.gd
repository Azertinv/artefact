extends CanvasLayer

onready var pause_menu = $Control/Centering/PauseMenu
onready var options = $Control/Centering/Options

export(bool) var can_pause: bool = false

func _ready():
	$Control.visible = false

func _input(event: InputEvent):
	if not can_pause:
		return
	if !$Control.visible and event.is_action_pressed("ui_cancel"):
		get_tree().set_input_as_handled()
		get_tree().paused = true
		$Control.visible = true
	elif $Control.visible and event.is_action_pressed("ui_cancel"):
		get_tree().set_input_as_handled()
		_on_OptionsGoBack_pressed()
		_on_Resume_pressed()

func _on_Resume_pressed():
	get_tree().paused = false
	$Control.visible = false

func _on_Options_pressed():
	pause_menu.visible = false
	options.visible = true

func _on_Quit_pressed():
	get_tree().quit()

func _on_OptionsGoBack_pressed():
	pause_menu.visible = true
	options.visible = false
