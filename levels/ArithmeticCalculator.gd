extends MarginContainer

func _ready():
	if !Save.has_section_key("ReportGui", "TutorialDone"):
		pass
#		var answer_box = $HBoxContainer/CenterContainer2/CalculatorReport/WhatIsEqual/Answer/AnswerBox
#		Indicator.indicate(answer_box)
#		$Dialog.play_dialog("ReportGui/Tutorial", OS.is_debug_build())
#		yield($Dialog, "dialog_completed")
#		if answer_box != null:
#			yield(answer_box, "mouse_entered")
#		Indicator.clear()
#		Save.set_value("ReportGui", "TutorialDone", true)

func _on_CalculatorReport_completed():
	$Dialog.play_dialog("ArithmeticCalculator/Done")
	yield($Dialog, "dialog_completed")
	get_tree().change_scene_to(load("res://levels/ArithmeticArtefact.tscn"))
