extends Node

func _ready():
	DialogScene.play_dialog("IntroCutscene")
	yield(DialogScene, "dialog_completed")
	get_tree().change_scene_to(load("res://levels/ArithmeticCalculator.tscn"))
