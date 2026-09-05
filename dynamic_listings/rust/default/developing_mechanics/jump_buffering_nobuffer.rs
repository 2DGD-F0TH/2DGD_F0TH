# #![allow(dead_code, unused)]
#
# struct Key;
# impl Key {
#     fn is_pressed(&self) -> bool { todo!() }
# }
# struct Controls {
#     jump: Key,
# }
# struct Player {
#     on_ground: bool,
# }
# struct Game {
#     controls: Controls,
#     player: Player,
# }
# impl Game {
fn update(&mut self, dt: f32) {
    // ...
    if self.controls.jump.is_pressed() {
        if self.player.on_ground {
            // Jump
        }
    }
    // ...
}
# }
