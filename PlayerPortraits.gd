extends HBoxContainer

func player_portraits():
	return get_node("PlayerPortraits")

func _ready():
	player_character_portraits(player_characters())
	
func player_character_portraits(player_characters: Array[Character]):

	for character: Character in player_characters:
		var portraitTexture: Texture = character.portrait
		var portraitTextureRect = TextureRect.new()
		portraitTextureRect.texture = portraitTexture
		player_portraits().add_child(portraitTextureRect)
	
func player_characters() -> Array[Character]:
	var player_characters := get_node("/root/Tabletop/PlayerCharacters").get_children() as Array[Character]
	return player_characters
