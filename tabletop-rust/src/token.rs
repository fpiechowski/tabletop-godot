use std::fmt::Display;

use godot::builtin::StringName;
use godot::engine::{Area2D, Camera2D, CharacterBody2D, CircleShape2D, CollisionShape2D, INode2D, InputEvent, InputEventMouseButton, Node2D, NodeExt, Sprite2D};
use godot::engine::global::MouseButton;
use godot::obj::{Gd, WithBaseField};
use godot::prelude::{Base, godot_api, GodotClass, NodePath, Vector2};

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct Token {
    character_node: NodePath,
    selected: bool,
    movement_target_position: Option<Vector2>,
    hovered: bool,
    base: Base<Node2D>,
}

#[godot_api]
impl Token {
    const SPEED: f32 = 50f32;

    #[func]
    fn sprite2d(&self) -> Gd<Sprite2D> {
        self.base().get_node_as::<Sprite2D>(NodePath::from("CharacterBody2D/Sprite2D"))
    }

    #[func]
    fn collision_shape(&self) -> Gd<CollisionShape2D> {
        self.base()
            .get_node_as::<CollisionShape2D>(NodePath::from("CharacterBody2D/CollisionShape2D"))
    }

    #[func]
    fn character_body(&self) -> Gd<CharacterBody2D> {
        self.base().get_node_as::<CharacterBody2D>(NodePath::from("CharacterBody2D"))
    }

    #[func]
    fn area2d(&self) -> Gd<Area2D> {
        self.base().get_node_as::<Area2D>(NodePath::from("Area2D"))
    }

    #[func]
    fn camera2d(&self) -> Gd<Camera2D> {
        self.base().get_node_as::<Camera2D>(NodePath::from("/root/Camera2D"))
    }

    #[func]
    fn on_mouse_enter(&mut self) {
        self.hovered = true
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        self.hovered = false
    }

    fn process_movement(&mut self, delta: f64) {
        if let Some(target) = self.movement_target_position {
            let position = self.base().get_position();
            if (target == self.base().get_position()) {
                println!("target reached");
                self.movement_target_position = None
            } else {
                let velocity = position.direction_to(target) * Self::SPEED;
                let space = velocity * delta as f32;
                self.character_body().move_and_collide(space);
                //println!("moving by {space}");
            }
        }
    }

    fn set_collision_shape_radius_from_texture(&mut self) {
        let token_size = self.sprite2d()
            .get_texture()
            .map(|txt| txt.get_size())
            .unwrap_or(Vector2::ZERO);
        println!("token_size = {:?}", token_size);

        let collision_shape = self.collision_shape();
        println!("collision_shape = {:?}", collision_shape);

        if let Some(new_shape) = collision_shape.get_shape().clone() {
            println!("new_shape = {:?}", new_shape);
            if let Ok(mut circle_shape) = new_shape.try_cast::<CircleShape2D>() {
                println!("circle_shape = {:?}", circle_shape);
                circle_shape.set_radius(1.5 * token_size.x)
            }
        }
    }

    fn select(&mut self) {
        self.selected = true;
    }
    fn deselect(&mut self) {
        self.selected = false;
    }
}

trait TokenInputFilter {
    fn input_selection(&mut self, event: &Gd<InputEvent>);

    fn input_movement(&mut self, event: &Gd<InputEvent>);
}

impl TokenInputFilter for Token {
    fn input_selection(&mut self, event: &Gd<InputEvent>) {
        if let Ok(e) = event.clone().try_cast::<InputEventMouseButton>() {
            let mouse_button = e.get_button_index();

            if (mouse_button == MouseButton::LEFT && e.is_pressed() && self.hovered) {
                self.select()
            }
        }
    }

    fn input_movement(&mut self, event: &Gd<InputEvent>) {
        if let Ok(e) = event.clone().try_cast::<InputEventMouseButton>() {
            let mouse_click_position = e.get_global_position();
            let mouse_button = e.get_button_index();

            if (mouse_button == MouseButton::RIGHT && e.is_released()) {
                println!("target set for mouse at {mouse_click_position}");

                self.movement_target_position = Some(mouse_click_position);
            }
        }
    }
}

#[godot_api]
impl INode2D for Token {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            character_node: NodePath::default(),
            movement_target_position: None,
            selected: false,
            hovered: false,
            base,
        }
    }

    fn ready(&mut self) {
        println!("token.ready");
        self.set_collision_shape_radius_from_texture();
        self.character_body().connect("mouse_entered".into(), self.base().callable("on_mouse_entered"));
        self.character_body().connect("mouse_exited".into(), self.base().callable("on_mouse_exited"));
    }

    fn process(&mut self, delta: f64) {
        self.process_movement(delta)
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if (self.selected()) {
            self.input_movement(&event)
        }

        self.input_selection(&event)
    }
}
