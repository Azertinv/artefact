extends Node

func pack_node_tree(node: Node) -> PackedScene:
	set_owners(node)
	var packed_answer = PackedScene.new()
	packed_answer.pack(node)
	return packed_answer

func set_owners(node: Node):
	set_owners_helper(node, node)

func set_owners_helper(node: Node, new_owner: Node):
	if node != new_owner:
		node.owner = new_owner
	for c in node.get_children():
		c.owner = new_owner
		set_owners_helper(c, new_owner)

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
