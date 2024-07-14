class Player:
    JUMP_VELOCITY = -12.0
    y_velocity = 0
    # ...

    def on_jump_key_pressed(self):
        """ The jump key has just been pressed (doesn't account the jump
        key being pressed from previous frames) """
        self.y_velocity = self.JUMP_VELOCITY

    def on_jump_key_released(self):
        # The jump key was just released, cut the y_speed so the jump is lower
        if self.y_velocity < self.JUMP_VELOCITY / 2:
            # The speed is higher than the cutoff speed (in absolute value)
            self.y_velocity = self.JUMP_VELOCITY / 2
