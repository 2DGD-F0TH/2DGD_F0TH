GRAVITY_ACCELERATION = 10
MAX_FALL_VELOCITY = 500
GRAVITY_FALL_MULTIPLIER = 1.5
# ...
# Are we Jumping?
if speed_y < 0:
    # Apply Gravity
    speed_y += GRAVITY_ACCELERATION
else:
    # We're falling, enhance gravity
    speed_y += GRAVITY_ACCELERATION * GRAVITY_FALL_MULTIPLIER
# Cap the fall speed
if speed_y > MAX_FALL_VELOCITY:
    speed_y = MAX_FALL_VELOCITY
# ...
