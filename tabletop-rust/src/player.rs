use godot::engine::{INode, Node};
use godot::obj::{Base};
use godot::prelude::{godot_api, GodotClass, GString};
use uuid::Uuid;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Player {
    id: Uuid,
    name: GString,
    base: Base<Node>,
}

#[godot_api]
impl Player {
    #[func]
    pub fn test(&mut self) {}
}

#[godot_api]
impl INode for Player {
    fn init(base: Base<Node>) -> Self {
        Player {
            id: Uuid::new_v4(),
            name: GString::new(),
            base,
        }
    }
}
