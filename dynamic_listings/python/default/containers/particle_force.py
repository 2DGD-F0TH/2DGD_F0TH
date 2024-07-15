class Particle:
    """
    This is a simple particle class, now it has some mass
    and a force application function
    """
    # ...
    mass: float = None

    def __init__(self, texture: Texture, position: Vector2, velocity: Vector2, acceleration: Vector2, lifespan: float = 2000, rotation: float = 0, a_vel: float = 0, a_accel: float = 0, mass: float = 1) -> None:
        # We prepare the particle for usage the same way as earlier
        # ...
        self.mass = mass

    # ...

    def applyForce(self, force: Vector2) -> None:
        # This function influences the acceleration by applying force
        da = force / self.mass
        self.acceleration += da
