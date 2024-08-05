GRAVITY_ACCELERATION: int = 10
# We consider the jump "peaking" when the speed is between -50 and 50
PEAKING_SPEED: int = 50
MAX_FALL_VELOCITY: int = 500
GRAVITY_FALL_MULTIPLIER: float = 1.5
PEAKING_MULTIPLIER: float = 0.5
# ...
# Are we jumping?
if (speed_y < - PEAKING_SPEED):
    # We're rising: apply Gravity Normally
    speed_y += GRAVITY_ACCELERATION
elif (speed_y > PEAKING_SPEED):
    # We're falling, enhance gravity
    speed_y += + GRAVITY_ACCELERATION * GRAVITY_FALL_MULTIPLIER
else:
    # Our jump is peaking, lower gravity to give more time
    speed_y += GRAVITY_ACCELERATION * PEAKING_MULTIPLIER

# Cap the fall speed
if (speed_y > MAX_FALL_VELOCITY):
    speed_y = MAX_FALL_VELOCITY
# ...
