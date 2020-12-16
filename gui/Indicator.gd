extends CanvasLayer

signal cleared

var queue = []
var indicated: Node

func _ready():
	$Box.visible = false
	set_process(false)

func clear() -> void:
	$AnimationPlayer.stop()
	$Box.visible = false
	indicated = null
	set_process(false)
	emit_signal("cleared")
	if not queue.empty():
		indicate(queue.pop_front())

func indicate(node: Control) -> void:
	if $Box.visible:
		queue.append(node)
		return
	$Box.visible = true
	indicated = node
	set_process(true)
	$AnimationPlayer.play("blink")

func _process(_delta):
	$Box.rect_global_position = indicated.rect_global_position
	$Box.rect_size = indicated.rect_size
