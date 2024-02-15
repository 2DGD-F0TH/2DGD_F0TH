let game_is_running = True;

// We initialize our dt at 1/60th of a second for the first loop
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
}
