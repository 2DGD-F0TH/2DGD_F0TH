def update(dt: float):
    # ...
    if controls.jump.is_pressed():
        if player.is_on_ground:
            player.jump()
    # ...
