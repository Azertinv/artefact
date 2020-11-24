extends HBoxContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)

var addr: int = 0

const BYTE_PER_LINE: int = 3
const LINE_COUNT: int = 9

var cached_byte_edits: Array = []
func cache_byte_edits() -> void:
	for i in range(BYTE_PER_LINE * LINE_COUNT):
		cached_byte_edits.append(
			get_node("Memory/Line"+str(i/BYTE_PER_LINE)+"/ByteEdit"+str(i%BYTE_PER_LINE)))
func get_byte_edit(index: int) -> Node:
	return cached_byte_edits[index]

func _ready() -> void:
	cache_byte_edits()
	if artefact == null:
		push_error("MemoryViewer not connected to Artefact")
		return
	addr = artefact.get_reg_value(0)
	print(addr)

func _input(event):
	if event is InputEventMouseButton:
		if event.is_action_pressed("dbg_mem_down"):
			addr += BYTE_PER_LINE
		if event.is_action_pressed("dbg_mem_up"):
			addr -= BYTE_PER_LINE

func _process(_delta) -> void:
	var bytes: PoolIntArray = artefact.get_mem_chunk(addr, BYTE_PER_LINE * LINE_COUNT)
	for i in range(BYTE_PER_LINE * LINE_COUNT):
		if i < bytes.size():
			get_byte_edit(i).visible = true
			get_byte_edit(i).value = bytes[i]
		else:
			get_byte_edit(i).visible = false
