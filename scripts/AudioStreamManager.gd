extends Node

var num_players = 8
var bus = "Master"

var available = []  # The available players.
var queue = []  # The queue of sounds to play.

func _ready():
	for i in num_players:
		var p = AudioStreamPlayer.new()
		add_child(p)
		available.append(p)
		p.connect("finished", self, "_on_stream_finished", [p])
		p.bus = bus

func _on_stream_finished(stream):
	available.append(stream)

func play(sound_path: String):
	queue.append(sound_path)

func _process(delta):
	if not queue.empty() and not available.empty():
		available[0].stream = load(queue.pop_front())
		available[0].play()
		available.pop_front()
