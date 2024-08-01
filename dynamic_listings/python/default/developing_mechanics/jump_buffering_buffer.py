# ...
JUMP_BUFFER_TIME: float = 5.0
# ...


def update(dt: float) -> None:
    # ...
    if controls.jump.is_pressed():
        player.has_buffered_jump = True
        player.jump_buffer_countdown = JUMP_BUFFER_TIME
    # Take note on how this piece is outside the "jump is pressed" section
    if player.has_buffered_jump:
        player.jump_buffer_countdown -= dt
    if player.is_on_ground:
        if player.jump_buffer_countdown > 0.0:
            player.jump()
            player.jump_buffer_countdown = 0.0
            player.has_buffered_jump = False
    # ...
