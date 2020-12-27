extends HBoxContainer

func _ready():
	PauseScreen.can_pause = false

func _on_StartGame_pressed():
	PauseScreen.can_pause = true
	get_tree().change_scene_to(load("res://cutscenes/IntroCutscene.tscn"))

func _on_Options_pressed():
	$StartMenu.visible = false
	$Options.visible = true

func _on_Quit_pressed():
	get_tree().quit()

func _on_OptionsGoBack_pressed():
	$StartMenu.visible = true
	$Options.visible = false
