@tool
extends Node

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var has = get_tree().get_root().get_node("Node2D/Camera2D")
	print("ready ", has) # true
	pass


func _get_configuration_warnings():
	var has = get_tree().get_root().get_node("Node2D/Camera2D")
	print("_get_configuration_warnings ", has) # false
	pass
