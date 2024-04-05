use godot::builtin::GString;
use godot::builtin::meta::GodotType;
use godot::engine::{INode, Node, NodeExt, Texture2D};
use godot::obj::{Base, NewGd, WithBaseField};
use godot::prelude::{Gd, godot_api, GodotClass, NodePath};
use uuid::Uuid;

use crate::identifiable::Identifiable;
use crate::player::Player;
use crate::token::Token;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Character {
    id: Uuid,
    #[export]
    name: GString,
    #[export]
    hp: i32,
    #[export]
    player_node_path: NodePath,
    token_path: NodePath,
    #[export]
    portrait: Gd<Texture2D>,
    base: Base<Node>,
}

impl Character {
    fn token(&self) -> Gd<Token> {
        self.base().get_node_as::<Token>(self.token_path.clone())
    }
}

trait PlayerControlled {
    fn player(&self) -> Gd<Player>;
}

impl PlayerControlled for Character {
    fn player(&self) -> Gd<Player> {
        self.base().get_node_as::<Player>(self.player_node_path.clone())
    }
}

pub(crate) enum CharacterType {
    PlayerCharacter {
        character: Gd<Character>,
        player: Gd<Player>,
    },

    NonPlayerCharacter {
        character: Gd<Character>
    },
}

#[godot_api]
impl INode for Character {
    fn init(base: Base<Node>) -> Self {
        Self {
            id: Uuid::new_v4(),
            hp: 0,
            name: GString::default(),
            player_node_path: NodePath::default(),
            portrait: Texture2D::new_gd(),
            token_path: NodePath::from("Token"),
            base,
        }
    }
}

impl Identifiable<Uuid> for Character {
    fn get_id(&self) -> &Uuid {
        &self.id
    }
}

