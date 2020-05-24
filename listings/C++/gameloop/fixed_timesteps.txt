// ...
float dt = 1.0/60.0;
bool game_is_running = true;

while(game_is_running){
    process_user_input();
    update_world(dt);
    draw();
}
//...
