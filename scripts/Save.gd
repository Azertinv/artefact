extends Node

const save_path = "user://save.cfg"
var save = null

var key_to_node = {}
var node_to_key = {}

func _enter_tree():
	save = ConfigFile.new()
	if save.load(save_path) != OK:
		_exit_tree()

func _exit_tree():
	save.save(save_path)

func reset():
	save = ConfigFile.new()
	_exit_tree()

func register_node(section: String, key: String, node: Node):
	if key_to_node.has(section+key):
		push_error(node.name+" is registering a duplicate save entry from original node "+key_to_node[section+key].name)
		breakpoint
	if node_to_key.has(node):
		push_error(node.name+" is registering a duplicate save entry from original node "+node_to_key[node].name)
		breakpoint
	key_to_node[section+key] = node
	node_to_key[node] = [section, key]

func get_value(section: String, key: String):
	return save.get_value(section, key)

func has_section_key(section: String, key: String) -> bool:
	return save.has_section_key(section, key)

func set_value(section: String, key: String, value):
	save.set_value(section, key, value)
