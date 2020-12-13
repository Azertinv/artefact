extends HBoxContainer

export(NodePath) var bg_color_rect_path
onready var bg_color_rect = get_node(bg_color_rect_path)

func _ready():
	visible = false
	bg_color_rect.visible = false

func _input(event: InputEvent):
	if not visible and event.is_action_pressed("ui_cancel"):
		get_tree().set_input_as_handled()
		get_tree().paused = true
		visible = true
		bg_color_rect.visible = true

func _on_StartGame_pressed():
	get_tree().paused = false
	visible = false
	bg_color_rect.visible = false

func _on_Options_pressed():
	$PauseMenu.visible = false
	$Options.visible = true

func _on_Quit_pressed():
	get_tree().quit()

func _on_OptionsGoBack_pressed():
	$PauseMenu.visible = true
	$Options.visible = false
