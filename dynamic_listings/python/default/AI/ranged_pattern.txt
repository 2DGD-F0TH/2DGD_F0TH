from random import randint


class Boss:
    def __init__(Player player):
        self.player = player
        self.player_too_close = False
        self.base_movement_velocity = 10
        self.too_close_space = 20
        self.velocity = Vector2D()
        self.position = Vector2D()

    def update(self, dt):
        # ...
        if (abs(self.player.position.x - self.position.x) < self.too_close_space):
            if (abs(self.player.position.y - self.position.y) < self.too_close_space):
                # The player is too close
                if (randint(1, 5) == 1):
                    # Add a bit of randomization
                    self.player_too_close = True

        # We're using a variable to preserve the "too close" state between frames
        if (self.player_too_close):
            # The player is too close, make some distance
            distance = self.position - player.position
            # Make it a direction
            direction = distance.normalize()
            # self is the direction the boss should go now, transfer it to velocity
            self.velocity = direction * self.base_movement_velocity

        # ...
        # The boss and player now have moved, let's see if they're far enough
        # ...
        if (abs(self.player.position.x - self.position.x) > self.too_close_space):
            if (abs(self.player.position.y - self.position.y) > self.too_close_space):
                # The player is far enough now
                self.player_too_close = False
