dt: float = 1.0/60.0
game_is_running: bool = True

while game_is_running:
    process_user_input()
    update_world(dt)
    draw()
