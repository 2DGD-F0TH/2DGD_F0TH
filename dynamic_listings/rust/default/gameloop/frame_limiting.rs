use std::time::Duration;
use std::thread::sleep;

let target_time = 1_000 / 60;
let mut game_is_running = true;

// We bootstrap dt to 1/60th of a second for the first frame
let mut dt = 1_000 / 60;

while game_is_running {
    // We get the system time in milliseconds
    // since implementation varies here i'll use a generic function name
    let begin = system_time_millis();
    process_user_input();
    update_world(dt);
    draw();
    let end = system_time_millis();
    // We update our dt
    dt = end - begin;
    let millis = u64::max(target_time - dt, 0);
    sleep(Duration::from_millis(millis));
}
