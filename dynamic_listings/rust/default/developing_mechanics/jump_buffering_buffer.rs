// ...
const JUMP_BUFFER_TIME: f32 = 5.;
// ...
fn update(&mut self, dt: f32) {
    //...
    if self.controls.jump.is_pressed() {
        self.player.has_buffered_jump = true;
        self.player.jump_buffer_countdown = Self::JUMP_BUFFER_TIME;
    }
    // Take note on how this piece is outside the "jump is pressed" section
    if self.player.has_buffered_jump {
        self.player.jump_buffer_countdown = self.player.jump_buffer_countdown - dt;
    }
    if self.player.on_ground {
        if self.player.jump_buffer_countdown > 0. {
            // Jump
            self.player.jump_buffer_countdown = 0.;
            self.player.has_buffered_jump = false;
        }
    }
    //...
}
