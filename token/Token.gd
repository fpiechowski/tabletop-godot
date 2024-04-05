extends CharacterBody2D

var selected: bool = false
var hovered: bool  = false
var movement_target_position: Vector2


func _ready():
	print("_ready")


func _process(delta):
	pass


func _input(event):
	if selected:
		input_movement(event)

	input_selection(event)


func _draw() -> void:
	if movement_target_position != null:
		draw_circle(movement_target_position, 20, Color.YELLOW)


func on_mouse_entered():
	print("hovered")

	
func on_mouse_exited():
	print("unhovered")



func input_movement(event):
	if event.is_action_released("move"):
		var mouse_click_position: Vector2 = event.get_global_position();

		var camera_transform: Transform2D = get_viewport().get_camera_2d().get_global_transform();
		var viewport_halfsize: Vector2 = get_viewport_rect().size / 2;
		var mouse_click_pos_in_world: Vector2 = camera_transform * mouse_click_position - viewport_halfsize;

		movement_target_position = mouse_click_pos_in_world;
		print("input_movement");
		queue_redraw();


func input_selection(event):
	if event.is_action_pressed("select"):
		if hovered:
			print("selected")			
			select()


func select():
	pass
