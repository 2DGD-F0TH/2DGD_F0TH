GRAVITY_ACCELERATION: int = 10
MAX_FALL_VELOCITY: int = 500
# ...
# Apply Gravity
speed_y += GRAVITY_ACCELERATION
# Cap the fall speed
if speed_y > MAX_FALL_VELOCITY:
    speed_y = MAX_FALL_VELOCITY
# ...
