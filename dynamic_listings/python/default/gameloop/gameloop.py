def game() -> None:
    game_is_running: bool = True
    while game_is_running:
        process_user_input()
        update_world()
        draw()
