class Emitter:
    """
    This is a simple particle emitter, it contains a list
    of particles and it updates and manages them
    """
    origin = None
    particles = None
    # Defines if this emitter streams continuously or only a burst of particles
    one_shot = False

    def __init__(self, location, one_shot=False):
        self.origin = location
        self.particles = []  # We prepare a list of particles, we'll use 8
        self.one_shot = one_shot

    def update(self, dt):
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
