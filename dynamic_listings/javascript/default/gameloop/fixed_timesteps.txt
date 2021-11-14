let dt = 1.0/60.0;
let game_is_running = True;

while (game_is_running){
    process_user_input();
    update_world(dt);
    draw();
}
