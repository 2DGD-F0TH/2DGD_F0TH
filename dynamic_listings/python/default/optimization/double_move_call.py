def update(dt):
    # We assume the presence of a "Vector" class
    vector_up = Vector(0, -1)
    vector_right = Vector(1, 0)
    # ...
    characterController.move(vector_up* dt)
    characterController.move(vector_right * dt)
    # ...
