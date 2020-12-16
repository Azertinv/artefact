extends MarginContainer

func _on_CalculatorReport_completed():
	get_tree().change_scene_to(load("res://levels/ArithmeticArtefact.tscn"))
