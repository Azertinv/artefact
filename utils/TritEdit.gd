extends Label

signal value_changed(new_value)

export(bool) var editable = true
var value: int = 0 setget set_value

func set_value(new_value: int):
	value = new_value
	update_text(false)

func get_drag_data(_position) -> bool:
	return true

func can_drop_data(_pos, _data) -> bool:
	update_value()
	return true

func _ready():
	set_process(false)

func update_text(emit: bool = true) -> void:
	if value == 0:
		text = "0"
	if value == 1:
		text = "1"
	if value == -1:
		text = "T"
	if emit:
		emit_signal("value_changed", value)

func update_just_value() -> void:
	if not editable:
		return
	var old_value = value
	if Input.is_action_just_pressed("trit_zero"):
		value = 0
	if Input.is_action_just_pressed("trit_one"):
		value = 1
	if Input.is_action_just_pressed("trit_tern"):
		value = -1
	if old_value != value:
		update_text()

func update_value() -> void:
	if not editable:
		return
	var old_value = value
	if Input.is_action_pressed("trit_zero"):
		value = 0
	if Input.is_action_pressed("trit_one"):
		value = 1
	if Input.is_action_pressed("trit_tern"):
		value = -1
	if old_value != value:
		update_text()

func _on_TritEdit_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.button_index == BUTTON_LEFT:
		accept_event()
		update_value()

func _on_TritEdit_mouse_entered() -> void:
	set_process(true)
func _on_TritEdit_mouse_exited() -> void:
	set_process(false)

func _process(_delta):
	if !Input.is_mouse_button_pressed(BUTTON_LEFT):
		update_just_value()
