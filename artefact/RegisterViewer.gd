extends VBoxContainer

export(NodePath) var artefact_path

onready var artefact = get_node(artefact_path)

enum Registers {
	PC = 0,
	SP,
	FLAGS,
	A,B,C,D,E,F,
}

func _ready() -> void:
	if artefact == null:
		push_error("RegisterViewer not connected to Artefact")
		return
	for r in get_children():
		r.connect("trits_changed", self, "_on_Register_trits_changed", [r])

func _process(_delta) -> void:
	for r in get_children():
		r.set_trits(artefact.get_reg_trits(Registers.get(r.name)))

func _on_Register_trits_changed(new_trits: Array, reg: Node) -> void:
	artefact.set_reg_trits(Registers.get(reg.name), PoolIntArray(new_trits))
