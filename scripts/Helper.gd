extends Node

func pack_node_tree(node: Node) -> Array:
	var result = []
	for c in node.get_children():
		result.append(pack_node_tree(c))
	return [node, result]

func unpack_node_tree(data: Array) -> Node:
	var node = data[0]
	for c in data[1]:
		node.add_child(unpack_node_tree(c))
	return node

#TODO FIXME make it support multiple events for 1 action
#func set_event_for_action(action, event):
#	InputMap.action_erase_events(action)
#	InputMap.action_add_event(action, event)
#
#func get_event_for_action(action):
#	var event = InputMap.get_action_list(action)
#	if event.size() != 1:
#		push_error("Invalid input map settings: "+action)
#		return -1
#	return event[0]
