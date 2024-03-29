import random

class Player:
    # ...
    def registerShootingObserver(callback):
        # Function used to register an observer that will be called when the
        # player shoots a projectile.
        pass


class JumpingBoss:
    # A boss that jumps when the player shoots. Sometimes.

    player_shot = False
    y_velocity = 0.0
    on_ground = False

    def __init__(self, x, y, player):
        # ...
        player.registerShootingObserver(self.setPlayerShot)
        # ...

    def setPlayerShot(self):
        # Sets a state that tells the AI that the player shot a bullet
        self.player_shot = True

    def jump(self):
        # Sets the boss velocity to -10, making it jump
        if self.on_ground:
            self.y_velocity = -10

    def update(self, dt):
        # ...
        if self.player_shot is True:
            if random.randint(1, 6) == 1:
                # Jump only 20% of the times the player shoots
                self.jump()

        # We reset player_shot to false, if we didn't the boss would jump
        # a lot more often than 20% of the time
        self.player_shot = False
        # ...
