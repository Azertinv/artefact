extends Node

var num_players = 8

var available = []  # The available players.
var queue = []  # The queue of sounds to play.

func _ready():
	for i in num_players:
		var p = AudioStreamPlayer.new()
		add_child(p)
		available.append(p)
		p.connect("finished", self, "_on_stream_finished", [p])
	play("res://assets/songs/bleeping-demo-by-kevin-macleod.ogg", "Music", PAUSE_MODE_PROCESS)

func _on_stream_finished(player):
	available.append(player)

func play(sound_path: String, bus: String = "Master", pause_mode: int = PAUSE_MODE_INHERIT):
	queue.append([sound_path, bus, pause_mode])

func _process(_delta):
	if not queue.empty() and not available.empty():
		var entry = queue.pop_front()
		available[0].stream = load(entry[0])
		available[0].bus = entry[1]
		available[0].pause_mode = entry[2]
		available[0].play()
		available.pop_front()
