extends Label

signal gui_value_changed(new_value)

export(bool) var editable: bool = true setget set_editable

var value: int = 0 setget set_value

func set_value(new_value: int) -> void:
	if value != new_value:
		value = new_value
		update_text(false)

func set_editable(new_value: bool):
	if new_value != editable:
		$WritePermIndicator.visible = new_value
		editable = new_value

func get_drag_data(_position) -> bool:
	return true

func can_drop_data(_pos, _data) -> bool:
	update_value()
	return true

func _ready():
	$WritePermIndicator.visible = editable
	set_process(false)

func update_text(emit: bool) -> void:
	if value == 0:
		text = "0"
	if value == 1:
		text = "1"
	if value == -1:
		text = "T"
	if emit:
		emit_signal("gui_value_changed", value)

# return true if an action was processed
func update_value() -> bool:
	if not editable:
		return false
	var processed = false
	var old_value = value
	if Input.is_action_pressed("trit_zero"):
		value = 0
		processed = true
	if Input.is_action_pressed("trit_one"):
		value = 1
		processed = true
	if Input.is_action_pressed("trit_tern"):
		value = -1
		processed = true
	if old_value != value:
		update_text(true)
	return processed

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton and event.button_index == BUTTON_LEFT:
		if update_value():
			accept_event()
