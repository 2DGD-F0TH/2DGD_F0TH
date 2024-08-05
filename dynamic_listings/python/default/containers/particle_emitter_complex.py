class Emitter:
    """
    This is a simple particle emitter, it contains a list
    of particles and it updates and manages them
    """
    origin: Vector2 = None
    particles: list[Particle] = []
    # Defines if this emitter streams continuously or only a burst of particles
    one_shot: bool = False

    def __init__(self, location: Vector2, one_shot: bool = False) -> None:
        self.origin = location
        self.particles = [Particle() for _ in range(8)]  # We prepare 8 particles
        self.one_shot = one_shot

    def update(self, dt: float) -> None:
        # Update the entire system, by updating each particle
        for particle in self.particles:
            if self.one_shot:
                if particle.is_dead():
                    continue
                else:
                    particle.update(dt)
            else:
                if particle.is_dead():
                    particle.reset()  # Resets the state of the particle
                    particle.setPosition(self.origin)
                particle.update(dt)
