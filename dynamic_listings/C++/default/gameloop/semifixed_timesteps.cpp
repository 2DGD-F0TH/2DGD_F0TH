float dt = 1.0/60.0;
bool game_is_running = true;

// We bootstrap frametime for 1/60th of a second for the first frame
float frametime = 1.0/60.0;

while(game_is_running){
    // We get the system time in milliseconds
    // since implementation varies here i'll use a generic function name
    float begin = get_system_time_millis();

    while(frametime > 0.0){
        float deltaTime = min(dt, frametime);
        process_user_input();
        update_world(dt);
        frametime = frametime - deltaTime;
        draw();
    }
    float end = get_system_time_millis();
    // We memorize how long this frame lasted
    frametime = end - begin;
}
