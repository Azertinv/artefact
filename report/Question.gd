extends HBoxContainer

export(String) var question_id = ""

var answer: String

func _ready():
	if question_id == "":
		push_error("NewQuestion spawned with no id")
		return
	var data = ReportLoader.questions[question_id]
	answer = data["answer"]
	Helper.init_format(self, data["format"], data["possibilities"])

#TODO
func is_good_answer() -> bool:
	return false

func get_answer() -> String:
	return ""

func mark_question_as_answered():
	pass

#TODO save system
