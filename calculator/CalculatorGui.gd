extends Node2D

onready var Op = $MarginContainer/VBoxContainer/HBoxContainer3/Op
onready var Rhs = $MarginContainer/VBoxContainer/HBoxContainer3/Rhs
onready var Lhs = $MarginContainer/VBoxContainer/Lhs

func _ready():
	for n in get_tree().get_nodes_in_group("calculator_buttons"):
		n.connect("button_pressed", self, "_on_CalculatorButton_pressed")
	update_display()

func update_display():
	Lhs.text = $Calculator.get_lhs()
	Op.text = $Calculator.get_op()
	Rhs.text = $Calculator.get_rhs()

func _on_CalculatorButton_pressed(button):
	$Calculator.push_button(button)
	update_display()
