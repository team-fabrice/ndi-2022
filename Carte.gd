extends Node2D

export var file : String

var font
var data
var color

var mouse_in = false
var can_grab = false
var grabbed_offset = Vector2()

func _input(event):
	if event is InputEventMouseButton and event.position and mouse_in:
		can_grab = event.pressed
		grabbed_offset = position - get_global_mouse_position()
		color = Color.blue
	else:
		color = Color.antiquewhite

func _process(delta):
	if Input.is_mouse_button_pressed(BUTTON_LEFT) and can_grab:
		position = get_global_mouse_position() + grabbed_offset

func _ready():
	font = DynamicFont.new()
	font.font_data = load("res://noto.ttf")
	var f = File.new()
	f.open("res://data/cards/" + file + ".json", File.READ)
	data = parse_json(f.get_as_text())

func _draw():
	draw_rect(Rect2(0, 0, 100, 150), color)
	draw_string(font, Vector2(10, 20), data["display_name"], Color(0, 0, 0))


func _on_mouse_entered():
	mouse_in = true

func _on_mouse_exited():
	mouse_in = false
