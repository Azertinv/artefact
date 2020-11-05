extends Node2D

func _ready():
	for n in get_tree().get_nodes_in_group("calculator_buttons"):
		n.connect("button_pressed", self, "_on_CalculatorButton_pressed")

func _on_CalculatorButton_pressed(button):
	$Calculator.push_button(button)
