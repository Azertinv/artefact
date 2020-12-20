extends CanvasLayer

signal dialog_completed

var did: int = 0
var nid: int = 0
var story_reader

onready var speaker_box = $DialogScene/VBoxContainer/NinePatchRect/Speaker
onready var speaker_label = $DialogScene/VBoxContainer/NinePatchRect/Speaker/Label
onready var dialog_label = $DialogScene/VBoxContainer/NinePatchRect/Dialog
onready var portraits = $DialogScene/VBoxContainer/PortraitAnchor/Portraits
onready var portrait_animation = $DialogScene/VBoxContainer/PortraitAnchor/AnimationPlayer

func _ready() -> void:
	$DialogScene.visible = false
	var story_reader_class = preload("res://addons/EXP-System-Dialog/Reference_StoryReader/EXP_StoryReader.gd")
	story_reader = story_reader_class.new()
	var story = preload("res://dialog/story_baked.tres")
	story_reader.read(story)

func play_dialog(record_name: String) -> void:
	did = story_reader.get_did_via_record_name(record_name)
	nid = story_reader.get_nid_via_exact_text(did, "<start>")
	play_next_node()
	$DialogScene.visible = true

func play_next_node() -> void:
	nid = story_reader.get_nid_from_slot(did, nid, 0)
	if story_reader.get_text(did, nid) == "<end>":
		emit_signal("dialog_completed")
		$DialogScene.visible = false
	else:
		play_node()

func _input(event: InputEvent) -> void:
	if not $DialogScene.visible:
		return
	if event.is_action_pressed("ui_accept"):
		get_tree().set_input_as_handled()
		play_next_node()
	elif event.is_action_pressed("cheat"):
		get_tree().set_input_as_handled()
		emit_signal("dialog_completed")
		$DialogScene.visible = false

func get_tagged_text(tag : String, text : String) -> String:
	var start_tag = "<" + tag + ">"
	var end_tag = "</" + tag + ">"
	var start_index = text.find(start_tag) + start_tag.length()
	return text.substr(start_index, text.find(end_tag) - start_index)

func play_node() -> void:
	var text: String = story_reader.get_text(did, nid)
	var speaker: String = get_tagged_text("speaker", text)
	var portrait: String = get_tagged_text("portrait", text)
	var dialog: String = get_tagged_text("dialog", text)
	if speaker == "":
		speaker_box.visible = false
	else:
		speaker_box.visible = true
		speaker_label.bbcode_text = "[center]" + speaker
	dialog_label.bbcode_text = dialog
	for c in portraits.get_children():
		if c.name == portrait:
			if not c.visible:
				portrait_animation.stop()
				portrait_animation.play("slide_in_portrait")
			c.visible = true
		else:
			c.visible = false
