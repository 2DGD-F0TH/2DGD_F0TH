def update(dt: float) -> None:
    # ...
    if controls.jump.is_pressed():
        if player.is_on_ground:
            player.jump()
    # ...
