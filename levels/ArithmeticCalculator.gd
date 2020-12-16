extends MarginContainer

func _on_CalculatorReport_completed():
	yield(get_tree().create_timer(2), "timeout")
	DialogScene.play_dialog("ArithmeticCalculator/Done")
	yield(DialogScene, "dialog_completed")
	get_tree().change_scene_to(load("res://levels/ArithmeticArtefact.tscn"))
