extends CanvasLayer

func _ready():
	$Box.visible = false

func clear() -> void:
	$AnimationPlayer.stop()
	$Box.visible = false

func indicate(node: Control) -> void:
	$Box.visible = true
	$Box.rect_global_position = node.rect_global_position
	$Box.rect_size = node.rect_size
	$AnimationPlayer.play("blink")
