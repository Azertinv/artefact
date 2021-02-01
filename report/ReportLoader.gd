extends Node

var questions = null
var answers = null

func _ready():
	var file = File.new()
	file.open("res://report_data.json", File.READ)
	var content = file.get_as_text()
	file.close()
	content = JSON.parse(content).result
	questions = content["questions"]
	answers = content["answers"]
