extends VBoxContainer

export(bool) var can_reset_save = true

var resolutions = [
	Vector2(1280, 720),
	Vector2(1366, 768),
	Vector2(1600, 900),
	Vector2(1920, 1080),
	Vector2(2560, 1440),
	Vector2(3200, 1800),
	Vector2(3840, 2160),
	Vector2(800, 600),
	Vector2(960, 720),
	Vector2(1280, 960),
	Vector2(1400, 1050),
	Vector2(1600, 1200),
]

func res_to_str(width, height = 0):
	if width is Vector2:
		return str(width[0]) + "x" + str(width[1])
	return str(width) + "x" + str(height)
func str_to_res(string):
	string = string.split("x")
	return Vector2(int(string[0]), int(string[1]))

func _ready():
	$Fullscreen/CheckBox.pressed = OS.window_fullscreen
	$Borderless/CheckBox.pressed = OS.window_borderless
	$Resolution/MenuButton.text = res_to_str(OS.window_size)
	var resolution_popup = $Resolution/MenuButton.get_popup()
	resolution_popup.connect("index_pressed", self, "_on_Resolution_pressed")
	for r in resolutions:
		resolution_popup.add_item(res_to_str(r))
	var bus_idx = AudioServer.get_bus_index("Master")
	$MasterLevel/HSlider.value = AudioServer.get_bus_volume_db(bus_idx)
	if not can_reset_save:
		$ResetSave.visible = false

func _on_Resolution_pressed(idx):
	var resolution_popup = $Resolution/MenuButton.get_popup()
	var new_resolution = resolution_popup.get_item_text(idx)
	$Resolution/MenuButton.text = resolution_popup.get_item_text(idx)
	new_resolution = str_to_res(new_resolution)
	get_viewport().size = new_resolution
	OS.window_size = new_resolution

func _on_Fullscreen_toggled(button_pressed):
	OS.window_fullscreen = button_pressed
func _on_Borderless_toggled(button_pressed):
	OS.window_borderless = button_pressed

func _on_MasterLevel_value_changed(value):
	var bus_idx = AudioServer.get_bus_index("Master")
	AudioServer.set_bus_volume_db(bus_idx, value)

func _on_ResetSave_confirmed_pressed():
	Save.reset()
