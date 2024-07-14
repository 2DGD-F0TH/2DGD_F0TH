def update(dt):
    vector_up = Vector(0, -1)
    vector_right = Vector(1, 0)
    # ...
    total_movement = vector_up + vector_right
    characterController.move(total_movement * dt)
    # ...
