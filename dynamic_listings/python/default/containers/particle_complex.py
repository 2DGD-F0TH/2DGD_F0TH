class Particle:
    """
    This is a simple particle class, it contains a reference to
    its texture, as well as some state
    """
    # The particle texture
    texture: Texture = None
    # Position, velocity and acceleration on the 2D plane
    position: Vector2 = None
    velocity: Vector2 = None
    acceleration: Vector2 = None
    # Lifespan of the particle
    lifespan: float = 2000
    # Current angle of rotation, and relative velocity and acceleration
    rotation: float = 0
    angular_velocity: float = 0
    angular_acceleration: float = 0

    # Initial Status, for resetting
    initial_velocity: Vector2 = None
    initial_rotation: float = 0
    initial_a_vel: float = 0
    initial_lifespan: float = 0

    def __init__(self, texture: Texture, position: Vector2, velocity: Vector2, acceleration: Vector2, lifespan: float = 2000, rotation: float = 0, a_vel: float = 0, a_accel: float = 0) -> None:
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
        self.angular_acceleration = a_accel

    def update(self, dt: float) -> None:
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

    def is_dead(self) -> bool:
        # Returns a boolean representing if the particle is dead
        return self.lifespan <= 0

    def reset(self) -> None:
        # This function resets the initial status of the particle
        self.velocity = self.initial_velocity
        self.rotation = self.initial_rotation
        self.a_vel = self.initial_a_vel
        self.lifespan = self.initial_lifespan
