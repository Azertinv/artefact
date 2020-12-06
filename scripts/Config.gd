extends Node

const config_path = "user://settings.cfg"
var config = null

func _enter_tree():
	config = ConfigFile.new()
	if config.load(config_path) == OK:
		get_viewport().size = config.get_value("display", "resolution")
		OS.window_size = config.get_value("display", "resolution")
		OS.window_fullscreen = config.get_value("display", "fullscreen")
		OS.window_borderless = config.get_value("display", "borderless")
	else:
		_exit_tree()

func _exit_tree():
	config.set_value("display", "resolution", OS.window_size)
	config.set_value("display", "fullscreen", OS.window_fullscreen)
	config.set_value("display", "borderless", OS.window_borderless)
	config.save(config_path)
