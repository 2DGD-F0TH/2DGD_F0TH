void game(){
    bool game_is_running = true;
    while(game_is_running){
        process_user_input();
        update_world();
        draw();
    }
}
