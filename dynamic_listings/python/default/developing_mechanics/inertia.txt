# ...
class Player:
    input_accel = Vector2()  # Defines the direction we are accelerating
    velocity = Vector2()  # Defines the direction and magnitude of our speed
    position = Vector2()  # Defines our current position, in (x,y) coordinates
    is_moving = False  # Tells us if we're moving
    MAX_SPEED = 50.0  # Maximum speed
    ACCEL = 15.0  # The acceleration rate
    DECEL = 30.0  # The deceleration rate

    def handle_input(self):
        # First of all, we need to zero the input_accel,
        # or we'll be working on "residual data"
        self.input_accel = Vector2.ZERO
        # Now we can handle movement
        if KEYBOARD.Left_Arrow_Pressed:
            self.input_accel.x = self.input_accel.x - 1
        if KEYBOARD.Right_Arrow_Pressed:
            self.input_accel.x = self.input_accel.x + 1
        if KEYBOARD.Down_Arrow_Pressed:
            self.input_accel.y = self.input_accel.y + 1
        if KEYBOARD.Right_Arrow_Pressed:
            self.input_accel.y = self.input_accel.y - 1
        # If any component of the acceleration vector isn't zero, we are moving
        if self.input_accel != Vector2.ZERO:
            self.is_moving = True

    def handle_movement(self, dt):
        if self.is_moving:
            # Vectors will take care of summing forces
            self.velocity = self.velocity + self.ACCEL * dt * self.input_accel
            # We need to clamp the speed, to avoid going too fast
            self.velocity.clamp(MAX_SPEED)
        else:
            # We are stopping, let's subtract the deceleration
            velocity_value = len(self.velocity) - self.DECEL * dt
            if velocity_value < 0:
                # If, After decelerating, we have a negative value, we need
                # to make it zero or the object will start moving backwards
                velocity_value = 0
            # We are just changing the length of the vector, so we can just
            # clamp its length
            self.velocity.clamp(velocity_value)

        # Now it's time to move the object
        self.position = self.position + self.velocity
