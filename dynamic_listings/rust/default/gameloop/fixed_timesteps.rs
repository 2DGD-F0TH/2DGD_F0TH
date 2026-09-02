# #![allow(unused)]
#
# fn process_user_input() {}
# fn update_world(_: u64) {}
# fn draw() {}
# fn main() {
// ...
let dt = 1_000 / 60;
let mut game_is_running = true;

while game_is_running {
    process_user_input();
    update_world(dt);
    draw();
}
//...
# }
