extends Node2D

export var file : String

var font
var data

func _ready():
	font = DynamicFont.new()
	font.font_data = load("res://noto.ttf")
	var f = File.new()
	f.open("res://data/cards/" + file + ".json", File.READ)
	data = parse_json(f.get_as_text())

func _draw():
	draw_rect(Rect2(0, 0, 100, 150), Color.blanchedalmond)
	draw_string(font, Vector2(10, 20), data["display_name"], Color(0, 0, 0))
