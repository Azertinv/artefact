extends Node

var text_lbl_scene = preload("res://utils/TextLabel.tscn")
var alien_lbl_scene = preload("res://utils/AlienLabel.tscn")
var answer_box_scene = preload("res://report/AnswerBox.tscn")
var number_edit_scene = preload("res://utils/NumberEdit.tscn")

enum {
	TAG_TEXT,
	TAG_ALIEN,
	TAG_ANSWER_BOX,
	TAG_NUMBER_EDIT,
}

func parse_format(format) -> Array:
	var result = []
	while format.length() > 0:
		var entry = []
		var end = format.find("%", 1)
		if end == -1:
			end = format.length()
		var start = 0
		var type = TAG_TEXT
		if format[0] == "%":
			start = 2
			match format[1]:
				"t":
					type = TAG_TEXT
				"a":
					type = TAG_ALIEN
				"b":
					type = TAG_ANSWER_BOX
				"d":
					type = TAG_NUMBER_EDIT
		entry.append(type)
		entry.append(format.substr(start, end - start))
		result.append(entry)
		format = format.substr(end)
	return result

func init_format(target, format, possibilities):
	var i: int = 0
	for entry in parse_format(format):
		match entry[0]:
			TAG_TEXT:
				var tmp = text_lbl_scene.instance()
				tmp.text = entry[1].strip_edges()
				target.add_child(tmp)
			TAG_ALIEN:
				var tmp = alien_lbl_scene.instance()
				tmp.text = entry[1].strip_edges()
				target.add_child(tmp)
			TAG_ANSWER_BOX:
				var tmp = answer_box_scene.instance()
				tmp.possible_answers = possibilities[i].duplicate()
				i += 1
				target.add_child(tmp)
			TAG_NUMBER_EDIT:
				target.add_child(number_edit_scene.instance())

func get_parent_report(node: Node) -> Node:
	while not node.is_in_group("report"):
		node = node.get_parent()
	return node
