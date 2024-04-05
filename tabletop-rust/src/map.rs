use godot::engine::Node2D;
use godot::prelude::{Base, godot_api, GodotClass, INode2D};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct WorldMap {
    base: Base<Node2D>
}

#[godot_api]
impl WorldMap {
    
}

#[godot_api]
impl INode2D for WorldMap {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleMap {
    base: Base<Node2D>
}

#[godot_api]
impl BattleMap {

}

#[godot_api]
impl INode2D for BattleMap {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}