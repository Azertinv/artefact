extends HBoxContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)

var addr: int = 0
var last_addr: int

const BYTE_PER_LINE: int = 3
const LINE_COUNT: int = 20

var cached_bytes: Array = []
func cache_bytes() -> void:
	for i in range(BYTE_PER_LINE * LINE_COUNT):
		cached_bytes.append(
			get_node("Memory/Line"+str(i/BYTE_PER_LINE)+"/ByteEdit"+str(i%BYTE_PER_LINE)))
func get_byte(index: int) -> Node:
	return cached_bytes[index]

var cached_addrs: Array = []
func cache_addrs() -> void:
	for i in range(LINE_COUNT):
		cached_addrs.append(
			get_node("Memory/Line"+str(i)+"/Address"))
func get_addr(index: int) -> Node:
	return cached_addrs[index]

func _ready() -> void:
	cache_bytes()
	cache_addrs()
	if artefact == null:
		push_error("MemoryViewer not connected to Artefact")
		breakpoint
	for i in range(BYTE_PER_LINE * LINE_COUNT):
		get_byte(i).connect("gui_value_changed", self, "_on_ByteEdit_gui_value_changed", [i])
	addr = artefact.get_reg_value(0)
	last_addr = 0

func _on_ByteEdit_gui_value_changed(new_value, _new_trits, index):
	artefact.mem_write(addr + index, PoolIntArray([new_value]))

func _input(event: InputEvent):
	if not visible:
		return
	if event is InputEventMouseButton:
		if event.is_action_pressed("dbg_scroll_down"):
			addr += BYTE_PER_LINE
		if event.is_action_pressed("dbg_scroll_up"):
			addr -= BYTE_PER_LINE

func goto(new_addr: int) -> void:
	if not visible:
		return
	addr = new_addr

func _process(_delta: float) -> void:
	var bytes: PoolIntArray = artefact.mem_read(addr, BYTE_PER_LINE * LINE_COUNT)
	for i in range(BYTE_PER_LINE * LINE_COUNT):
		if i < bytes.size():
			if get_byte(i).visible != true:
				get_byte(i).visible = true
			if bytes[i] != get_byte(i).value:
				get_byte(i).value = bytes[i]
		else:
			if get_byte(i).visible != false:
				get_byte(i).visible = false
	if last_addr != addr:
		for i in range(LINE_COUNT):
			get_addr(i).value = addr + i * BYTE_PER_LINE
		last_addr = addr
