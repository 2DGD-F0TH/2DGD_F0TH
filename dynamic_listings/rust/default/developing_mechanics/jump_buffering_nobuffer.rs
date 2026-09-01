fn update(&mut self, dt: f32) {
    // ...
    if self.controls.jump.is_pressed() {
        if self.player.on_ground {
            // Jump
        }
    }
    // ...
}
