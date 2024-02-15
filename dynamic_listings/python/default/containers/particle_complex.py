class Particle:
    """
    This is a simple particle class, it contains a reference to
    its texture, as well as some state
    """
    # The particle texture
    texture = None
    # Position, velocity and acceleration on the 2D plane
    position = None
    velocity = None
    acceleration = None
    # Lifespan of the particle
    lifespan = None
    # Current angle of rotation, and relative velocity and acceleration
    rotation = None
    angular_velocity = None
    angular_acceleration = None

    # Initial Status, for resetting
    initial_velocity = None
    initial_rotation = None
    initial_a_vel = None
    initial_lifespan = None

    def __init__(self, texture, position, velocity, acceleration,
                 lifespan=2000, rotation=0, a_vel=0, a_accel=0):
        # We prepare the particle for usage
        self.texture = texture
        self.position = position
        self.velocity = velocity
        self.acceleration = acceleration
        self.lifespan = lifespan  # About 2 seconds by default
        # We also prepare the reset variables: the position will be set
        # by the emitter
        self.initial_lifespan = lifespan
        self.initial_velocity = velocity
        self.initial_a_vel = a_vel
        self.initial_rotation = rotation

    def update(self, dt):
        # We update the velocity (assuming dt is in milliseconds)
        self.velocity += self.acceleration
        # Then the position
        self.position = self.position + self.velocity * dt
        # Then the rotation
        self.angular_velocity += self.angular_acceleration
        # Wrap to zero when at 360 degrees
        self.rotation = (self.rotation + self.angular_velocity * dt) % 360
        # Now we update the lifespan of the particle
        self.lifespan -= dt

    def is_dead(self):
        # Returns a boolean representing if the particle is dead
        return self.lifespan <= 0

    def reset(self):
        # This function resets the initial status of the particle
        self.velocity = self.initial_velocity
        self.rotation = self.initial_rotation
        self.a_vel = self.initial_a_vel
        self.lifespan = self.initial_lifespan
