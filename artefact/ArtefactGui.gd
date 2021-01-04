extends MarginContainer

export(String, FILE, "*.json") var program_path = ""

func _ready():
	if program_path != "":
		load_program_from_file(program_path)
	if !Save.has_section_key("ArtefactGui", "TutorialDone"):
		artefact_tutorial()

func artefact_tutorial():
	$Dialog.play_dialog("ArtefactGui/Tutorial", false)
	yield($Dialog, "dialog_completed")
	Save.set_value("ArtefactGui", "TutorialDone", true)

func load_program_from_file(path: String) -> void:
	var file = File.new()
	file.open(path, File.READ)
	var content = file.get_as_text()
	file.close()
	$Artefact.load_program_from_json(content)

var in_diag = false
func _input(event: InputEvent):
	if not in_diag and OS.is_debug_build() and event.is_action_pressed("cheat"):
		get_tree().set_input_as_handled()
		in_diag = true
		var diag = FileDialog.new()
		diag.current_dir = "res://levels/programs/"
		diag.mode = FileDialog.MODE_OPEN_FILE
		diag.filters = PoolStringArray(["*.json"])
		add_child(diag)
		diag.popup(Rect2(1920/2-500/2, 1080/2-500/2, 500, 500))
		program_path = yield(diag, "file_selected")
		load_program_from_file(program_path)
		diag.queue_free()
		in_diag = false
