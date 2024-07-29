import random
from typing import Callable


class Player:
    # ...
    def registerShootingObserver(callback: Callable) -> None:
        # Function used to register an observer that will be called when the
        # player shoots a projectile.
        ...


class JumpingBoss:
    # A boss that jumps when the player shoots. Sometimes.

    player_shot: bool = False
    y_velocity: float = 0.0
    on_ground: bool = False

    def __init__(self, x: int, y: int, player: Player) -> None:
        # ...
        player.registerShootingObserver(self.setPlayerShot)
        # ...

    def setPlayerShot(self) -> None:
        # Sets a state that tells the AI that the player shot a bullet
        self.player_shot = True

    def jump(self) -> None:
        # Sets the boss velocity to -10, making it jump
        if self.on_ground:
            self.y_velocity = -10

    def update(self, dt: float) -> None:
        # ...
        if self.player_shot is True:
            if random.randint(1, 6) == 1:
                # Jump only 20% of the times the player shoots
                self.jump()

        # We reset player_shot to false, if we didn't the boss would jump
        # a lot more often than 20% of the time
        self.player_shot = False
        # ...
