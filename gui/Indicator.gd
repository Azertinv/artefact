extends CanvasLayer

signal cleared

var queue = []

func _ready():
	$Box.visible = false

func clear() -> void:
	$AnimationPlayer.stop()
	$Box.visible = false
	emit_signal("cleared")
	if not queue.empty():
		indicate(queue.pop_front())

func indicate(node: Control) -> void:
	if $Box.visible:
		queue.append(node)
		return
	$Box.visible = true
	$Box.rect_global_position = node.rect_global_position
	$Box.rect_size = node.rect_size
	$AnimationPlayer.play("blink")
