class Player:
    JUMP_VELOCITY = -12.0
    y_speed = 0
    # ...

    def onJumpKeyPressed(self):
        """ The jump key has just been pressed (doesn't account the jump
        key being pressed from previous frames) """
        self.y_speed = self.JUMP_VELOCITY

    def onJumpKeyReleased(self):
        # The jump key was just released, cut the y_speed so the jump is lower
        if self.y_speed < self.JUMP_VELOCITY / 2:
            # The speed is higher than the cutoff speed (in absolute value)
            self.y_speed = self.JUMP_VELOCITY / 2
