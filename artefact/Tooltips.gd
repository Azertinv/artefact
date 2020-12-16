extends PanelContainer

func _ready():
	TooltipManager.connect("tooltip_changed", self, "set_tooltip")

func set_tooltip(tooltip: String) -> void:
	$VBoxContainer/Label.bbcode_text = "[center]" +tooltip
