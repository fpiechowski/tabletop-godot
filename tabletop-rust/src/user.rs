use godot::builtin::GString;
use godot::engine::{INode, Node};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};
use uuid::Uuid;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct User {
    id: Uuid,
    #[export]
    name: GString,
    base: Base<Node>
}

#[godot_api]
impl INode for User {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: GString::default(),
            base
        }
    }
}