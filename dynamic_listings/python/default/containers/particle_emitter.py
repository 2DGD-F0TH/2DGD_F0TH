class Emitter:
    """
    This is a simple particle emitter, it contains a list
    of particles and it updates and manages them
    """
    origin: Vector2 = None
    particles: list[Particle] = None

    def __init__(self, location: Vector2):
        self.origin = location
        self.particles = []  # We prepare a list of particles

    def update(self, dt: float):
        # Update the entire system, by updating each particle
        for particle in self.particles:
            if not particle.is_dead():
                particle.update(dt)
