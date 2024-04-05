use godot::engine::{Node, NodeExt};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{Array, Gd, godot_api, GodotClass, INode};

use crate::player::Player;
use crate::user::User;

#[derive(GodotClass)]
#[class(base = Node)]
struct Tabletop {
    base: Base<Node>,
}

#[godot_api]
impl INode for Tabletop {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

#[godot_api]
impl Tabletop {
    #[func]
    fn tabletop(&self) -> Gd<Tabletop> {
        return self.base().get_node_as::<Tabletop>("/root/Tabletop");
    }

    #[func]
    fn players(&self) -> Array<Gd<Player>> {
        self.tabletop().get_node("Players".into()).unwrap().get_children()
            .iter_shared()
            .filter_map(|child| child.try_cast::<Player>().ok())
            .collect()
    }

    #[func]
    fn users(&self) -> Array<Gd<User>> {
        self.tabletop().get_node("Users".into()).unwrap().get_children()
            .iter_shared()
            .filter_map(|child| child.try_cast::<User>().ok())
            .collect()
    }
}