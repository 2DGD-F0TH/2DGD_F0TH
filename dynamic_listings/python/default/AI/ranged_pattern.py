from random import randint


class Boss:
    def __init__(self, player: Player) -> None:
        self.player: Player = player
        self.player_too_close: bool = False
        self.base_movement_velocity: float = 10
        self.too_close_space: float = 20
        self.velocity: Vector2 = Vector2()
        self.position: Vector2 = Vector2()

    def update(self, dt: float) -> None:
        # ...
        if (abs(self.player.position.x - self.position.x) < self.too_close_space) and \
            (abs(self.player.position.y - self.position.y) < self.too_close_space):
                # The player is too close
                if (randint(1, 5) == 1):
                    # Add a bit of randomization
                    self.player_too_close = True

        # We're using a variable to preserve the "too close" state between frames
        if (self.player_too_close):
            # The player is too close, make some distance
            distance: Vector2 = self.position - player.position
            # Make it a direction
            direction: Vector2 = distance.normalize()
            # self is the direction the boss should go now, transfer it to velocity
            self.velocity = direction * self.base_movement_velocity

        # ...
        # The boss and player now have moved, let's see if they're far enough
        # ...
        if (abs(self.player.position.x - self.position.x) > self.too_close_space) and\
            (abs(self.player.position.y - self.position.y) > self.too_close_space):
                # The player is far enough now
                self.player_too_close = False
