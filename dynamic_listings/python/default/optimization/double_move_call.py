def update(dt: float) -> None:
    # We assume the presence of a "Vector2" class
    vector_up: Vector2 = Vector2(0, -1)
    vector_right: Vector2 = Vector2(1, 0)
    # ...
    character_controller.move(vector_up* dt)
    character_controller.move(vector_right * dt)
    # ...
