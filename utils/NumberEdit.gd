extends LineEdit

signal gui_value_changed(new_value)

var regex = RegEx.new()

func _ready():
	regex.compile("^\\-?[0-9]*$")

var old_text: String = ""

func set_text(new_text: String) -> void:
	var old_caret_position: int = caret_position
	text = new_text
	old_text = text
	caret_position = old_caret_position

func _on_NumberEdit_text_changed(new_text: String) -> void:
	var old_caret_position: int = caret_position
	if regex.search(new_text):
		text = new_text
		old_text = text
		caret_position = old_caret_position
		emit_signal("gui_value_changed", text)
	else:
		text = old_text
		caret_position = old_caret_position - 1

func get_number() -> String:
	if text == "":
		return text
	else:
		return str(int(text))
