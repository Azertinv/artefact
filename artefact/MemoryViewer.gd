extends VBoxContainer

export(NodePath) var artefact_path
onready var artefact = get_node(artefact_path)

var addr = 0

func get_byte_edit(index) -> Node:
	return get_node("ByteEdit"+str(index))

func _ready():
	if artefact == null:
		push_error("MemoryViewer not connected to Artefact")
		return
	addr = artefact.get_reg_value(0)
	print(addr)

func _process(_delta):
	var bytes = artefact.get_mem_chunk(addr)
	for i in range(7):
		if i < bytes.size():
			get_byte_edit(i).visible = true
			get_byte_edit(i).value = bytes[i]
		else:
			get_byte_edit(i).visible = false
