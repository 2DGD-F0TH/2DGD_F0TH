GRAVITY_ACCELERATION = 10
MAX_FALL_VELOCITY = 500
GRAVITY_FALL_MULTIPLIER = 1.5
-- ...
-- Are we jumping?
if (speed_y < 0) then
    -- Apply Gravity Normally
    speed_y = speed_y + GRAVITY_ACCELERATION
else
    -- We're falling, enhance gravity
    speed_y = speed_y + GRAVITY_ACCELERATION * GRAVITY_FALL_MULTIPLIER
end
-- Cap the fall speed
if (speed_y > MAX_FALL_VELOCITY) then
    speed_y = MAX_FALL_VELOCITY
end
-- ...
