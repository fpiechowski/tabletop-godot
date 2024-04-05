use godot::engine::global::MouseButton;
use godot::obj::{Base, WithBaseField};
use godot::prelude::{Camera2D, godot_api, GodotClass, ICamera2D, Input, Vector2};

#[derive(GodotClass)]
#[class(base = Camera2D)]
struct Camera {
    last_mouse_position: Vector2,
    base: Base<Camera2D>,
}

#[godot_api]
impl Camera {
}

#[godot_api]
impl ICamera2D for Camera {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            last_mouse_position: Vector2::ZERO,
            base
        }
    }

    fn process(&mut self, delta: f64) {
        let mut input = Input::singleton();

        if input.is_mouse_button_pressed(MouseButton::MIDDLE) {
            let  camera_movement =  (self.last_mouse_position - self.base().get_viewport().unwrap().get_mouse_position()) / delta as f32;
            println!("{camera_movement}");

            let current_position = self.base().get_position();

            self.base_mut().set_position((current_position + camera_movement) * delta as f32);
        }

        self.last_mouse_position = self.base().get_viewport().unwrap().get_mouse_position();
    }
}
