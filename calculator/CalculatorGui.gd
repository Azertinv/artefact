extends VBoxContainer

onready var Op = $Current/Op
onready var Rhs = $Current/Rhs
onready var Lhs = $Current/Lhs

onready var LastOp = $Last/Op
onready var LastRhs = $Last/Rhs
onready var LastLhs = $Last/Lhs

func _ready():
	for n in get_tree().get_nodes_in_group("calculator_buttons"):
		n.connect("button_pressed", self, "_on_CalculatorButton_pressed")
	update_display()

func update_display() -> void:
	Lhs.text = $Calculator.get_lhs()
	Op.text = $Calculator.get_op()
	Rhs.text = $Calculator.get_rhs()
	LastLhs.text = $Calculator.get_last_lhs()
	LastOp.text = $Calculator.get_last_op()
	LastRhs.text = $Calculator.get_last_rhs()

func _on_CalculatorButton_pressed(button: int) -> void:
	$Calculator.push_button(button)
	update_display()
