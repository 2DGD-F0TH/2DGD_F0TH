class Emitter:
    """
    This is a simple particle emitter, it contains a list
    of particles and it updates and manages them
    """
    origin: Vector2 = None
    particles: list[Particle] = []

    def __init__(self, location: Vector2) -> None:
        self.origin = location
        self.particles = [Particle() for _ in range(8)]  # We prepare 8 particles

    def update(self, dt: float) -> None:
        # Update the entire system, by updating each particle
        for particle in self.particles:
            if not particle.is_dead():
                particle.update(dt)
