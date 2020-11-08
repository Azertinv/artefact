extends LineEdit

var regex = RegEx.new()

func _ready():
	regex.compile("^\\-?[0-9]*$")

var old_text = ""

func _on_NumberEdit_text_changed(new_text: String) -> void:
	var old_caret_position = caret_position
	if regex.search(new_text):
		text = new_text
		old_text = text
		caret_position = old_caret_position
	else:
		text = old_text
		caret_position = old_caret_position - 1

func get_number() -> String:
	if text == "":
		return text
	else:
		return str(int(text))
