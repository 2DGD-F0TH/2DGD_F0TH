# #![allow(dead_code, unused)]
#
# fn system_time_millis() -> u64 { todo!() }
# fn process_user_input() {}
# fn update_world(_: u64) {}
# fn draw() {}
# fn main() {
let dt = 1_000 / 60;
let mut game_is_running = true;

// We bootstrap frame_time for 1/60th of a second for the first frame
let mut frame_time = 1_000 / 60;

while game_is_running {
    // We get the system time in milliseconds
    // since implementation varies here i'll use a generic function name
    let begin = system_time_millis();

    while frame_time > 0 {
        let delta_time = u64::min(dt, frame_time);
        process_user_input();
        update_world(dt);
        frame_time = frame_time - delta_time;
        draw();
    }
    let end = system_time_millis();
    // We memorize how long this frame lasted
    frame_time = end - begin;
}
# }
