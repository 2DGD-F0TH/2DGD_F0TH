class Player:
    # We assume the presence of a "Vector2" class
    speed: Vector2 = Vector2(0, 0)
    needs_update: bool = False
    # ...

    def get_input(self) -> None:
        # ...
        if right_key.is_pressed:
            speed = speed + Vector2(1, 0)  # Move right
            self.needs_update = True
            # ...
        if up_key.is_pressed:
            speed = speed + Vector2(0, -100)  # Move up (jump)
            self.needs_update = True
        # ...

    def update(self, dt: float) -> None:
        if self.needs_update:
            # Do Update instructions
            # ...
