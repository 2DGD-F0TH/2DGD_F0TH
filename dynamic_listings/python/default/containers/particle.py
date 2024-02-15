class Particle:
    """
    This is a simple particle class, it contains a reference to
    its texture, as well as some state
    """
    texture = None
    position = None
    velocity = None
    acceleration = None
    lifespan = None

    def __init__(self, texture, position, velocity,
                 acceleration, lifespan=2000):
        # We prepare the particle for usage
        self.texture = texture
        self.position = position
        self.velocity = velocity
        self.acceleration = acceleration
        self.lifespan = lifespan  # About 2 seconds by default

    def update(self, dt):
        # We update the velocity (assuming dt is in milliseconds)
        self.velocity += self.acceleration
        # Then the position
        self.position = self.position + self.velocity * dt
        # Now we update the lifespan of the particle
        self.lifespan -= dt

    def is_dead(self):
        # Returns a boolean representing if the particle is dead
        return self.lifespan <= 0

    def setPosition(self, position):
        # Sets the particle position
        self.position = position
