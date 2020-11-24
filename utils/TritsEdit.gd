extends HBoxContainer

signal gui_value_changed(new_value, new_trits)

export(int) var width = 9

enum {
	STATE_TRITS,
	STATE_DECIMAL,
}

var state = STATE_TRITS
var value: int = 0 setget set_value, get_value

var cached_trit_edits: Array = []
func cache_trit_edits() -> void:
	for i in range(width):
		cached_trit_edits.append(get_node("TritEdit"+str(i)))
func get_trit_edit(index) -> Node:
	return cached_trit_edits[index]

func _ready() -> void:
	cache_trit_edits()
	$NumberEdit.visible = false
	for i in range(width):
		get_trit_edit(i).connect("gui_value_changed", self, "_on_TritEdit_gui_value_changed")

func _gui_input(event):
	if event is InputEventMouseButton:
		if event.pressed and event.button_index == BUTTON_RIGHT:
			if state == STATE_TRITS:
				state = STATE_DECIMAL
				update_decimal_display()
				for i in range(width):
					get_trit_edit(i).visible = false
				$NumberEdit.visible = true
			elif state == STATE_DECIMAL:
				state = STATE_TRITS
				for i in range(width):
					get_trit_edit(i).visible = true
				$NumberEdit.visible = false

func update_decimal_display() -> void:
	if state != STATE_DECIMAL:
		return
	if str(value) != $NumberEdit.text:
		$NumberEdit.set_text(str(value))

#update value based on trit's value
func update_value() -> void:
	value = 0
	for i in range(width):
		value += get_trit_edit(i).value * int(pow(3, i))
	update_decimal_display()

#update trits based on value
func update_trits() -> void:
	var remainder = value
	for i in range(width):
		var trit = remainder % 3
		if trit < 0:
			trit += 3
		if trit == 2:
			trit = -1
		get_trit_edit(i).value = trit
		remainder -= trit
		remainder /= 3
	update_decimal_display()

func _on_NumberEdit_gui_value_changed(new_value: String) -> void:
	set_value(int(new_value))
	emit_signal("gui_value_changed", value, get_trits())

func _on_TritEdit_gui_value_changed(_new_value: int) -> void:
	update_value()
	emit_signal("gui_value_changed", value, get_trits())

func set_value(new_value: int) -> void:
	value = new_value
	update_trits()

func get_value() -> int:
	return value

func set_trits(new_trits: Array) -> void:
	for i in range(width):
		get_trit_edit(i).value = new_trits[i]
	update_value()

func get_trits() -> Array:
	var result = []
	for i in range(width):
		result.append(get_trit_edit(i).value)
	return result
