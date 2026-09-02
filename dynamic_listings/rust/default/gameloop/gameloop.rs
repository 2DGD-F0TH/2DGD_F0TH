# #![allow(dead_code, unused)]
# fn process_user_input() {}
# fn update_world() {}
# fn draw() {}
fn game() {
    let mut game_is_running = true;
    while game_is_running {
        process_user_input();
        update_world();
        draw();
    }
}
