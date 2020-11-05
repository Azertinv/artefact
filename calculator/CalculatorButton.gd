extends Button

signal button_pressed(button)

enum BUTTON {
	Zero = 0,
	One = 1,
	Tern = 2,
	Equal = 3,
	Add = 4,
	Sub = 5,
	Mul = 6,
	Div = 7,
}

export(BUTTON) var action = BUTTON.Zero

func _on_CalculatorButton_pressed():
	emit_signal("button_pressed", action)
