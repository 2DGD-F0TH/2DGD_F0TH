struct Player {
    speed: Vector2D,
    needs_update: bool,
}

impl Player {
    fn input(&mut self) {
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
