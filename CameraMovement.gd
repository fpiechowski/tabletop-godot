extends Camera2D

var dragging = false
var drag_start_pos
@export var drag_damping = 1.0

@export var zoom_speed = 0.1
var max_zoom = Vector2(2.0, 2.0)
var min_zoom = Vector2(0.5, 0.5)

func _process(delta):
	if Input.is_action_pressed("scroll"):
		if not dragging:
			dragging = true
			drag_start_pos = get_viewport().get_mouse_position()
	else:
		if dragging:
			dragging = false

	if dragging:
		var current_pos = get_viewport().get_mouse_position()
		var drag_delta = (current_pos - drag_start_pos) * drag_damping
		offset_camera(drag_delta)
		drag_start_pos = current_pos
		
		# Get the mouse wheel input
	var zoom_delta = 0
	if Input.is_action_just_released("zoom_in"):
		zoom_delta = 1
	if Input.is_action_just_released("zoom_out"):
		zoom_delta = -1

	# If the mouse wheel was scrolled
	if zoom_delta != 0:
		# Get the mouse position in world coordinates
		var mouse_pos = get_global_mouse_position()

		# Calculate the zoom factor
		var zoom_factor = 1.0 + zoom_delta * zoom_speed

		# Apply zoom factor
		zoom = clamp(zoom * zoom_factor, min_zoom, max_zoom)

		

func offset_camera(delta):
	position -= delta
