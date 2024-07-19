def update(dt: float) -> None:
    vector_up: Vector2 = Vector2(0, -1)
    vector_right: Vector2 = Vector2(1, 0)
    # ...
    total_movement: Vector2 = vector_up + vector_right
    character_controller.move(total_movement * dt)
    # ...
