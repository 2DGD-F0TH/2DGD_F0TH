class Particle:
    """
    This is a simple particle class, it contains a reference to
    its texture, as well as some state
    """
    texture: Texture = None
    position: Vector2 = None
    velocity: Vector2 = None
    acceleration: Vector2 = None
    lifespan: float = None

    def __init__(self, texture: Texture, position: Vector2, velocity: Vector2, acceleration: Vector2, lifespan: float = 2000):
        # We prepare the particle for usage
        self.texture = texture
        self.position = position
        self.velocity = velocity
        self.acceleration = acceleration
        self.lifespan = lifespan  # About 2 seconds by default

    def update(self, dt: float):
        # We update the velocity (assuming dt is in milliseconds)
        self.velocity += self.acceleration
        # Then the position
        self.position = self.position + self.velocity * dt
        # Now we update the lifespan of the particle
        self.lifespan -= dt

    def is_dead(self):
        # Returns a boolean representing if the particle is dead
        return self.lifespan <= 0

    def setPosition(self, position: Vector2):
        # Sets the particle position
        self.position = position
