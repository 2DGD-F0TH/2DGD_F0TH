class Emitter:
    """
    This is a simple particle emitter, it contains a list
    of particles and it updates and manages them
    """
    origin = None
    particles = None

    def __init__(self, location):
        self.origin = location
        self.particles = []  # We prepare a list of particles, we'll use 8

    def update(self, dt):
        # Update the entire system, by updating each particle
        for particle in self.particles:
            if not particle.is_dead():
                particle.update(dt)
