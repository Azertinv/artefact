extends MarginContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)

var addr: int = 0
var last_addr: int
var last_bytes: PoolIntArray

const MAX_BYTES_PER_LINE: int = 3
const LINE_COUNT: int = 5

var cached_code_lines: Array = []
func cache_code_lines() -> void:
	for i in range(LINE_COUNT):
		cached_code_lines.append(get_node("Disassembly/CodeLine"+str(i)))
func get_code_line(index: int) -> Node:
	return cached_code_lines[index]

func _ready() -> void:
	if artefact == null:
		push_error("MemoryViewer not connected to Artefact")
		breakpoint
	cache_code_lines()
	addr = artefact.get_reg_value(0)

func _input(event: InputEvent):
	if not visible:
		return
	if event is InputEventMouseButton:
		if event.is_action_pressed("dbg_scroll_down"):
			addr += get_code_line(0).size
		if event.is_action_pressed("dbg_scroll_up"):
			addr -= get_code_line(0).size # fix me need to disas 3 bytes up and then go down

func goto(new_addr: int) -> void:
	if not visible:
		return
	addr = new_addr

func _process(_delta: float) -> void:
	var bytes: PoolIntArray = artefact.mem_read(addr, MAX_BYTES_PER_LINE * (LINE_COUNT + 1))
	if last_bytes != bytes or last_addr != addr:
		var i = 0
		var l = 0
		while l < LINE_COUNT:
			i += get_code_line(l).update_from_buffer(bytes, i, addr + i)
			l += 1
		last_bytes = bytes
		last_addr = addr
