extends Node

const config_path = "user://settings.cfg"
var config = null

var saved_bus_level = [
	"Master",
	"Music",
]

func _enter_tree():
	config = ConfigFile.new()
	if config.load(config_path) == OK:
		OS.window_size = config.get_value("display", "resolution")
		OS.window_fullscreen = config.get_value("display", "fullscreen")
		OS.window_borderless = config.get_value("display", "borderless")
		for b in saved_bus_level:
			var bus_idx = AudioServer.get_bus_index(b)
			AudioServer.set_bus_volume_db(bus_idx, config.get_value("sound", b))
	else:
		_exit_tree()

func _exit_tree():
	config.set_value("display", "resolution", OS.window_size)
	config.set_value("display", "fullscreen", OS.window_fullscreen)
	config.set_value("display", "borderless", OS.window_borderless)
	for b in saved_bus_level:
		var bus_idx = AudioServer.get_bus_index(b)
		config.set_value("sound", b, AudioServer.get_bus_volume_db(bus_idx))
	config.save(config_path)
