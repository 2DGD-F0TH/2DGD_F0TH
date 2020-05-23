def update(dt):
    # We assume the presence of a "Vector" class
    vector_up = Vector(0, -1)
    vector_right = Vector(1, 0)
    # ...
    characterController.Move(vector_up* dt)
    characterController.Move(vector_right * dt)
    # ...
