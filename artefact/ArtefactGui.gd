extends MarginContainer

export(String, FILE, "*.json") var program_path = ""

func _ready():
	if program_path != "":
		load_program_from_file(program_path)

func load_program_from_file(path: String) -> void:
	var file = File.new()
	file.open(path, File.READ)
	var content = file.get_as_text()
	file.close()
	$Artefact.load_program_from_json(content)
