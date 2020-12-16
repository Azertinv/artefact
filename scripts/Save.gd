extends Node

const save_path = "user://save.cfg"
var save = null

func _enter_tree():
	save = ConfigFile.new()
	if save.load(save_path) != OK:
		_exit_tree()

func _exit_tree():
	save.save(save_path)

func reset():
	save = ConfigFile.new()
	_exit_tree()

func get_value(section: String, key: String):
	return save.get_value(section, key)

func has_section_key(section: String, key: String) -> bool:
	return save.has_section_key(section, key)

func set_value(section: String, key: String, value):
	save.set_value(section, key, value)
