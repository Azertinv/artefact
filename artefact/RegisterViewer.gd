extends VBoxContainer

export(NodePath) var artefact_path: NodePath
onready var artefact: Node = get_node(artefact_path)

var registers = []
func cache_register_nodes() -> void:
	registers.append($PC)
	registers.append($SP)
	registers.append($FLAGS)
	registers.append($A)
	registers.append($B)
	registers.append($C)
	registers.append($D)
	registers.append($E)
	registers.append($F)

func _ready() -> void:
	cache_register_nodes()
	if artefact == null:
		push_error("RegisterViewer not connected to Artefact")
		return
	for i in range(9):
		registers[i].connect("gui_value_changed", self, "_on_Register_gui_value_changed", [i])

func _process(_delta) -> void:
	for i in range(9):
		registers[i].set_trits(artefact.get_reg_trits(i))

func _on_Register_gui_value_changed(_new_value: int, new_trits: Array, reg: int) -> void:
	artefact.set_reg_trits(reg, PoolIntArray(new_trits))
