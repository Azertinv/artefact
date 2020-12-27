extends Node

func _ready():
	$Dialog.play_dialog("IntroCutscene")
	yield($Dialog, "dialog_completed")
	get_tree().change_scene_to(load("res://levels/ArithmeticCalculator.tscn"))
