extends CanvasLayer

signal cleared()

onready var box = $Box

var indicated: Node

func _ready():
	box.visible = false
	set_process(false)

func clear() -> void:
	$AnimationPlayer.stop()
	box.visible = false
	indicated = null
	set_process(false)
	emit_signal("cleared")

func indicate(node: Control, blink: bool = true) -> void:
	box.visible = true
	indicated = node
	_process(0) # avoid the stray indicator for 1 frame
	set_process(true)
	$AnimationPlayer.stop()
	if blink:
		$AnimationPlayer.play("blink")

func _process(_delta):
	if indicated != null:
		box.rect_global_position = indicated.rect_global_position
		box.rect_size = indicated.rect_size
	else:
		clear()
