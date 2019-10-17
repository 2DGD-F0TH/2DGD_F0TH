# ...
jumpBufferTime = 5.0
# ...


def update(dt):
    # ...
    if controls.jump.isPressed():
        player.hasBufferedJump = True
        player.jumpBufferCountdown = jumpBufferTime
    # Take note on how this piece is outside the "jump is pressed" section
    if player.hasBufferedJump:
        player.jumpBufferCountdown -= dt
    if player.on_ground:
        if player.jumpBufferCountdown > 0.0:
            player.jump()
            player.jumpBufferCountdown = 0.0
            player.hasBufferedJump = False
    # ...
