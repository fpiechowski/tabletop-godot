use godot::engine::{INode, Node, NodeExt};
use godot::obj::{Base, WithBaseField};
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
    hp: i32,
    #[export]
    player_node_path: NodePath,
    token_path: NodePath,
    base: Base<Node>,
}

impl Character {
    fn player(&self) -> Gd<Player> {
        self.base().get_node_as::<Player>(self.player_node_path.clone())
    }

    fn token(&self) -> Gd<Token> {
        self.base().get_node_as::<Token>(self.token_path.clone())
    }
}

#[godot_api]
impl INode for Character {
    fn init(base: Base<Node>) -> Self {
        Self {
            id: Uuid::new_v4(),
            hp: 0,
            player_node_path: NodePath::default(),
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

