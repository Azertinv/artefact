extends Node

signal tooltip_changed(new_tooltip)

var tooltip: String = "" setget set_tooltip

func set_tooltip(new_tooltip):
	tooltip = new_tooltip
	emit_signal("tooltip_changed", tooltip)
