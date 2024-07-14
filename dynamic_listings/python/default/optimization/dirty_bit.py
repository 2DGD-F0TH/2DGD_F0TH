class Player:
    # We assume the presence of a "Vector" class
    speed = Vector(0, 0)
    needs_update = False
    # ...

    def get_input():
        # ...
        if right_key.is_pressed:
            speed = speed + Vector(1, 0)  # Move right
            needs_update = True
            # ...
        if up_key.is_pressed:
            speed = speed + Vector(0, -100)  # Move up (jump)
            needs_update = True
        # ...

    def update(dt):
        if needs_update:
            # Do Update instructions
            # ...
