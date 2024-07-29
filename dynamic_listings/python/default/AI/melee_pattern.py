from random import randint


class Boss:
    def __init__(self, player: Player) -> None:
        self.player: Player = player
        self.player_too_far: bool = False
        self.base_movement_velocity: float = 10
        self.too_far_space: float = 30
        self.velocity: Vector2 = Vector2()
        self.position: Vector2 = Vector2()

    def update(self, dt: float) -> None:
        # ...
        if (abs(self.player.position.x - self.position.x) > self.too_far_space) and\
            (abs(self.player.position.y - self.position.y) > self.too_far_space):
                # The player is too close
                if randint(1, 5) == 1:
                    # Add a bit of randomization
                    self.player_too_far = True

        # We're using a variable to preserve the "too far" state between frames
        if self.player_too_far:
            # The player is too far, close in
            distance: Vector2 = self.player.position - self.position
            # Make it a direction
            direction: Vector2 = distance.normalize()
            # self is the direction the boss should go now, transfer it to velocity
            self.velocity = direction * self.base_movement_velocity

        # ...
        # The boss and player now have moved, let's see if they're close enough
        # ...
        if (abs(self.player.position.x - self.position.x) < self.too_far_space) and\
            (abs(self.player.position.y - self.position.y) < self.too_far_space):
                # The player is close enough now
                self.player_too_far = False
