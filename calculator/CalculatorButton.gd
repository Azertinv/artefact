extends Button

signal button_pressed(button)

enum BUTTON {
	Zero = 0,
	One,
	Tern,
	Equal,
	Add,
	Sub,
	Mul,
	Div,
}

export(BUTTON) var action = BUTTON.Zero

func _on_CalculatorButton_pressed():
	emit_signal("button_pressed", action)
