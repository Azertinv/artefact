extends HBoxContainer

export(String) var answer_id = ""

signal pressed()

var format: String
var possibilities: Array
var interactable = true setget set_interactable

func _ready() -> void:
	mouse_filter = MOUSE_FILTER_PASS
	if answer_id != "":
		var data = ReportLoader.answers[answer_id]
		format = data["format"]
		if data.has("possibilities"):
			possibilities = data["possibilities"]
	# if the answer has already been initialized (from a PackedScene)
	# skip the format initialization
	if get_child_count() == 0:
		Helper.init_format(self, format, possibilities)
	set_interactable(interactable)

func set_interactable(new_value: bool) -> void:
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
			emit_signal("pressed")
