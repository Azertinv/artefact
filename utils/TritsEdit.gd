extends HBoxContainer

signal gui_value_changed(new_value, new_trits)
signal gui_left_click()
signal gui_double_click()

export(int) var width = 9
export(int, LAYERS_3D_RENDER) var perm setget set_perm

enum {
	STATE_TRITS,
	STATE_DECIMAL,
}

var state = STATE_TRITS
var value: int = 0 setget set_value

onready var number_edit = $NumberEdit

var cached_trit_edits: Array = []
func cache_trit_edits() -> void:
	for i in range(width):
		cached_trit_edits.append(get_node("TritEdit"+str(i)))
func get_trit_edit(index) -> Node:
	return cached_trit_edits[index]

func _ready() -> void:
	cache_trit_edits()
	number_edit.visible = false
	number_edit.rect_min_size.x = get_trit_edit(0).rect_size.x * width
	for i in range(width):
		get_trit_edit(i).connect("gui_value_changed", self, "_on_TritEdit_gui_value_changed")
	set_perm(perm)

var last_left_click = 0
func _gui_input(event) -> void:
	if event is InputEventMouseButton and event.pressed:
		if event.button_index == BUTTON_RIGHT:
			if state == STATE_TRITS:
				state = STATE_DECIMAL
				update_decimal_display()
				for i in range(width):
					get_trit_edit(i).visible = false
				number_edit.visible = true
			elif state == STATE_DECIMAL:
				state = STATE_TRITS
				for i in range(width):
					get_trit_edit(i).visible = true
				number_edit.visible = false
		elif event.button_index == BUTTON_LEFT:
			emit_signal("gui_left_click")
			var left_click = OS.get_ticks_msec()
			if left_click - last_left_click < 200:
				emit_signal("gui_double_click")
			last_left_click = left_click

func update_decimal_display() -> void:
	if state != STATE_DECIMAL:
		return
	if str(value) != number_edit.text:
		number_edit.set_text(str(value))

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

func set_perm(new_perm: int) -> void:
	perm = new_perm
	for i in range(width):
		get_trit_edit(i).editable = new_perm & 1
		new_perm = new_perm >> 1

func set_value(new_value: int) -> void:
	if value != new_value:
		value = new_value
		update_trits()

func set_trits(new_trits: Array) -> void:
	var updated = false
	for i in range(width):
		if get_trit_edit(i).value != new_trits[i]:
			get_trit_edit(i).value = new_trits[i]
			updated = true
	if updated:
		update_value()

func get_trits() -> Array:
	var result = []
	for i in range(width):
		result.append(get_trit_edit(i).value)
	return result
