use godot::builtin::{GString, PackedStringArray, Vector2};
use godot::engine::{HBoxContainer, IHBoxContainer, NinePatchRect, PackedScene, try_load};
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass, NodePath};

use crate::character::Character;

#[derive(GodotClass)]
#[class(tool, base = HBoxContainer)]
struct PlayerCharacterPortraits {
    #[export]
    player_characters: NodePath,
    #[export]
    players: NodePath,
    errors: PackedStringArray,
    base: Base<HBoxContainer>,
}

#[godot_api]
impl PlayerCharacterPortraits {
    fn add_player_portraits_nodes(&mut self) {
        self.player_characters()
            .iter()
            .for_each(|player_character| self.instantiate_player_character_portrait(player_character));
    }

    fn instantiate_player_character_portrait(&mut self, character: &Gd<Character>) {
        if let Ok(packed_scene) = try_load::<PackedScene>("res://PlayerCharacterPortrait.tscn") {
            if let Some(scene) = packed_scene.instantiate() {
                if let Ok(mut node) = scene.try_cast::<NinePatchRect>() {
                    node.set_texture(character.bind().get_portrait());
                    node.set_custom_minimum_size(Vector2::new(128.0, 128.0));
                    node.set_size(Vector2::new(128.0, 128.0));

                    self.base_mut().add_child(node.upcast());
                }
            }
        }
    }

    fn player_characters(&mut self) -> Vec<Gd<Character>> {
        if let Some(node) = self.base().get_node(self.player_characters.clone()) {
            return node
                .get_children()
                .iter_shared()
                .filter_map(|node| node.try_cast::<Character>().ok())
                .collect();
        } else {
            self.errors.push(GString::from("No PlayerCharacters node set"))
        }

        return Vec::default();
    }
}

#[godot_api]
impl IHBoxContainer for PlayerCharacterPortraits {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            player_characters: NodePath::default(),
            players: NodePath::default(),
            errors: PackedStringArray::default(),
            base,
        }
    }

    fn ready(&mut self) {
        self.add_player_portraits_nodes()
    }

    fn get_configuration_warnings(&self) -> PackedStringArray {
        return self.errors.clone();
    }
}

