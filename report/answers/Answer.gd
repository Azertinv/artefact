class_name Answer
extends Control

signal pressed(answer)
var interactable = true setget set_interactable

func _ready() -> void:
	mouse_filter = MOUSE_FILTER_PASS
	set_interactable(interactable)

func set_interactable(new_value: bool) -> void:
	if interactable != new_value:
		var new_filter = MOUSE_FILTER_IGNORE
		if new_value:
			new_filter = MOUSE_FILTER_PASS
		for c in get_children():
			c.mouse_filter = new_filter
		interactable = new_value

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			#if we are not interactable we discard the event before the
			#children AnswerBox can process it
			if not interactable:
				accept_event()
#			print("Answer: "+str(event))
			emit_signal("pressed", self)
