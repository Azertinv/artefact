extends MarginContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)
export(NodePath) var memory_viewer_path: NodePath
onready var memory_viewer: Node = get_node(memory_viewer_path)

export(String, MULTILINE) var tooltip = "RegisterViewer"

var registers = []
func cache_register_nodes() -> void:
	registers.append($VBoxContainer/PC)
	registers.append($VBoxContainer/SP)
	registers.append($VBoxContainer/FLAGS)
	registers.append($VBoxContainer/A)
	registers.append($VBoxContainer/B)
	registers.append($VBoxContainer/C)
	registers.append($VBoxContainer/D)
	registers.append($VBoxContainer/E)
	registers.append($VBoxContainer/F)

func _ready() -> void:
	cache_register_nodes()
	if artefact == null:
		push_error("RegisterViewer not connected to Artefact")
		breakpoint
	if memory_viewer == null:
		push_error("RegisterViewer not connected to MemoryViewer")
		breakpoint
	var reg_perm = artefact.get_reg_perm()
	for i in range(9):
		registers[i].connect("gui_value_changed", self, "_on_Register_gui_value_changed", [i])
		registers[i].connect("gui_double_click", self, "_on_Register_gui_double_click", [i])
		registers[i].perm = reg_perm[i]

func _process(_delta) -> void:
	for i in range(9):
		var new_value = artefact.get_reg_value(i)
		if registers[i].value != new_value:
			registers[i].value = new_value

func _on_Register_gui_double_click(reg: int) -> void:
	memory_viewer.goto(registers[reg].value)

func _on_Register_gui_value_changed(_new_value: int, new_trits: Array, reg: int) -> void:
	artefact.set_reg_trits(reg, PoolIntArray(new_trits))

func _on_RegisterViewer_mouse_entered():
	TooltipManager.tooltip = tooltip
