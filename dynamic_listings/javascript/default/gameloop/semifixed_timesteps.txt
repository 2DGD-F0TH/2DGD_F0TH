let dt = 1.0/60.0;
let game_is_running = true;

// We bootstrap frametime for 1/60th of a second for the first frame
let frametime = 1.0/60.0;

while (game_is_running){
    // We get the system time in milliseconds
    let begin = Date.now();

    while (frametime > 0.0){
        let deltaTime = min(dt, frametime);
        process_user_input();
        update_world(deltaTime);
        frametime = frametime - deltaTime;
    }
    draw();
    let end = Date.now();
    // We memorize how long this frame lasted
    frametime = end - begin;
}
