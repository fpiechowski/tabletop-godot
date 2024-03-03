extends CharacterBody2D


const SPEED = 300.0
const JUMP_VELOCITY = -400.0

var target: Vector2



func _physics_process(delta):
	var direction = (target - global_position).normalized()
	if direction:
		velocity = direction * SPEED
	else:
		velocity = Vector2(move_toward(velocity.x, 0, SPEED), move_toward(velocity.y, 0, SPEED))

	move_and_slide()

func _input(event):
	if(event is InputEventMouseButton):
		if(event.button_index == MOUSE_BUTTON_RIGHT && event.is_released()):
			target = event.global_position
