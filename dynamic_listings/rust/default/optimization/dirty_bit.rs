# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
#
# enum Key {
#     Right,
#     Up,
# }
# impl Key {
#     fn is_pressed(&self) -> bool { todo!() }
# }
# #[derive(Default)]
# struct Keyboard;
# impl Keyboard {
#     fn get(&self, _: Key) -> Key { todo!() }
# }
struct Player {
    speed: Vector2D,
    needs_update: bool,
}

impl Player {
    fn input(&mut self) {
#       let Keyboard = Keyboard::default();
        // ...
        if Keyboard.get(Key::Right).is_pressed() {
            self.speed += Vector2D::new(1., 0.); // Move right
            self.needs_update = true;
        }
        // ...
        if Keyboard.get(Key::Up).is_pressed() {
            self.speed += Vector2D::new(0., -100.); // Move up (jump)
            self.needs_update = true;
        }
        // ...
    }

    fn update(&mut self, dt: f32) {
        if self.needs_update {
            // Do Update instructions
            // ...
            //
        }
    }
}
