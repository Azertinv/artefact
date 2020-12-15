extends HBoxContainer

func _ready():
	AudioStreamManager.play("res://assets/songs/bleeping-demo-by-kevin-macleod-from-filmmusic-io.ogg")

func _on_StartGame_pressed():
	get_tree().change_scene_to(preload("res://levels/ArithmeticCalculator.tscn"))

func _on_Options_pressed():
	$StartMenu.visible = false
	$Options.visible = true

func _on_Quit_pressed():
	get_tree().quit()

func _on_OptionsGoBack_pressed():
	$StartMenu.visible = true
	$Options.visible = false
