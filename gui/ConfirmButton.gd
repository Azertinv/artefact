extends Button

signal confirmed_pressed

enum {
	STATE_NORMAL,
	STATE_CONFIRM,
}

var state = STATE_NORMAL
var old_text: String = ""

func _ready():
	connect("pressed", self, "_on_self_pressed")

func _on_self_pressed():
	if state == STATE_NORMAL:
		old_text = text
		text = "Are you sure ?"
		state = STATE_CONFIRM
	elif state == STATE_CONFIRM:
		text = old_text
		state = STATE_NORMAL
		emit_signal("confirmed_pressed")
