let targetTime = 1.0/60.0;
let game_is_running = True;

// We bootstrap dt to 1/60th of a second for the first frame
let dt = 1.0/60.0;

while (game_is_running){
    // We get the system time in milliseconds
    let begin = Date.now();
    process_user_input();
    update_world(dt);
    draw();
    let end = Date.now();
    // We update our dt
    dt = end - begin;
    // We assume we have a single-thread sleep() function
    sleep(Math.max(targetTime - dt, 0));
}
