class Particle:
    """
    This is a simple particle class, now it has some mass
    and a force application function
    """
    # ...
    mass = None

    def __init__(self, texture, position, velocity, acceleration,
                 lifespan=2000, rotation=0, a_vel=0, a_accel=0, mass=1):
        # We prepare the particle for usage the same way as earlier
        # ...
        self.mass = mass

    # ...

    def applyForce(self, force):
        # This function influences the acceleration by applying force
        da = force / self.mass
        self.acceleration += da
