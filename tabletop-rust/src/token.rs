use std::fmt::{Debug, Formatter};

use godot::engine::{CharacterBody2D, CircleShape2D, CollisionShape2D, ICharacterBody2D, InputEvent, InputEventMouseButton, NodeExt, ShaderMaterial, Sprite2D};
use godot::obj::{Gd, WithBaseField};
use godot::prelude::{Base, godot_api, GodotClass, NodePath, ToGodot, Vector2};

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Token {
    #[export]
    character: NodePath,
    selected: bool,
    movement_target_position: Option<Vector2>,
    hovered: bool,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Token {
    const SPEED: f32 = 200f32;

    #[func]
    fn sprite2d(&self) -> Gd<Sprite2D> {
        self.base().get_node_as::<Sprite2D>(NodePath::from("Sprite2D"))
    }

    #[func]
    fn collision_shape(&self) -> Gd<CollisionShape2D> {
        self.base().get_node_as::<CollisionShape2D>(NodePath::from("CollisionShape2D"))
    }

    #[func]
    fn on_mouse_entered(&mut self) {
        self.hovered = true;
        println!("{:?}.on_mouse_entered()", self)
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        self.hovered = false
    }

    fn process_movement(&mut self, delta: f64) {
        if let Some(target) = self.movement_target_position {
            let position = self.base().get_global_position();
            if target.distance_to(position) < 5f32 {
                self.movement_target_position = None
            } else {
                let velocity = position.direction_to(target) * Self::SPEED;
                let space = velocity * delta as f32;
                self.base_mut().move_and_collide(space);
            }
        }
    }

    fn set_collision_shape_radius_from_texture(&mut self) {
        let token_size = self.sprite2d()
            .get_texture()
            .map(|txt| txt.get_size())
            .unwrap_or(Vector2::ZERO);

        let collision_shape = self.collision_shape();

        if let Some(new_shape) = collision_shape.get_shape().clone() {
            if let Ok(mut circle_shape) = new_shape.try_cast::<CircleShape2D>() {
                circle_shape.set_radius(1.5 * token_size.x)
            }
        }
    }

    fn select(&mut self) {
        self.selected = true;

        if let Some(mat) = self.sprite2d().get_material() {
            if let Ok(mut shader_material) = mat.try_cast::<ShaderMaterial>() {
                shader_material.set_shader_parameter("width".into(), 2f32.to_variant())
            }
        }

        println!("{:?}.select()", self)
    }
    fn deselect(&mut self) {
        self.selected = false;

        if let Some(mat) = self.sprite2d().get_material() {
            if let Ok(mut shader_material) = mat.try_cast::<ShaderMaterial>() {
                shader_material.set_shader_parameter("width".into(), 0f32.to_variant())
            }
        }
    }
}

#[godot_api]
impl ICharacterBody2D for Token {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            movement_target_position: None,
            selected: false,
            hovered: false,
            character: NodePath::default(),
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.process_movement(delta)
    }

    fn ready(&mut self) {
        self.set_collision_shape_radius_from_texture();

        let on_mouse_entered = self.base().callable("on_mouse_entered");
        self.base_mut().connect("mouse_entered".into(), on_mouse_entered);

        let on_mouse_exited = self.base().callable("on_mouse_exited");
        self.base_mut().connect("mouse_exited".into(), on_mouse_exited);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.selected {
            self.input_movement(&event)
        }

        self.input_selection(&event)
    }
}

trait TokenInputFilter {
    fn input_selection(&mut self, event: &Gd<InputEvent>);

    fn input_movement(&mut self, event: &Gd<InputEvent>);
}

impl TokenInputFilter for Token {
    fn input_selection(&mut self, event: &Gd<InputEvent>) {
        if event.is_action_pressed("select".into()) {
            if self.hovered {
                self.select()
            }
        }
    }

    fn input_movement(&mut self, event: &Gd<InputEvent>) {
        if event.is_action_released("move".into()) {
            if let Ok(e) = event.clone().try_cast::<InputEventMouseButton>() {
                let mouse_click_position = e.get_global_position();

                let camera_transform = self.base().get_viewport().unwrap().get_camera_2d().unwrap().get_global_transform();
                let viewport_halfsize = self.base().get_viewport_rect().size / 2f32;
                let mouse_click_pos_in_world = camera_transform * mouse_click_position - viewport_halfsize;

                self.movement_target_position = Some(mouse_click_pos_in_world);
                println!("{:?}.input_movement()", self);
                self.base_mut().queue_redraw();
            }
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({})", self.base().get_name())
    }
}