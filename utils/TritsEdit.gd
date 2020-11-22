extends HBoxContainer

signal trits_changed(new_trits)

export(int) var width = 9

func get_trit_edit(index) -> Node:
	return get_node("TritEdit"+str(index))

func _ready() -> void:
	for i in range(width):
		get_trit_edit(i).connect("value_changed", self, "_on_TritEdit_value_changed")

func _on_TritEdit_value_changed(_new_value) -> void:
	emit_signal("trits_changed", get_trits())

func set_trits(new_trits: Array) -> void:
	for i in range(width):
		get_trit_edit(i).value = new_trits[i]

func get_trits() -> Array:
	var result = []
	for i in range(width):
		result.append(get_trit_edit(i).value)
	return result

func get_value() -> int:
	var value = 0
	for i in range(width):
		value += get_trit_edit(i).value * int(pow(3, i))
	return value
