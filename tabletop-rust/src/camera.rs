use godot::builtin::Vector2;
use godot::engine::{InputEvent, InputEventMouseButton};
use godot::engine::global::MouseButton;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{Camera2D, godot_api, GodotClass, ICamera2D, Input};

#[derive(GodotClass)]
#[class(base=Camera2D)]
struct Camera {
    base: Base<Camera2D>
}

#[godot_api]
impl Camera  {
}

#[godot_api]
impl ICamera2D for Camera {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn process(&mut self, delta: f64) {
        let mut input = Input::singleton();

        if input.is_mouse_button_pressed(MouseButton::MIDDLE) {
            let last_mouse_velocity = input.get_last_mouse_velocity();
            println!("{last_mouse_velocity}");
            let camera_movement = last_mouse_velocity * delta as f32;
            let current_position = self.base().get_position();
            self.base_mut().set_position(current_position - camera_movement);
        }
    }
}