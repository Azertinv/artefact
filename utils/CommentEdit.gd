extends HBoxContainer

onready var label = $Label
onready var line_edit = $LineEdit

func _ready() -> void:
	label.text = line_edit.text
	line_edit.visible = false

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed and event.button_index == BUTTON_LEFT:
			label.visible = false
			line_edit.visible = true
			line_edit.grab_focus()

func _on_LineEdit_text_changed(new_text: String) -> void:
	label.text = new_text

func _on_LineEdit_text_entered(new_text: String) -> void:
	label.text = new_text
	line_edit.visible = false
	label.visible = true
