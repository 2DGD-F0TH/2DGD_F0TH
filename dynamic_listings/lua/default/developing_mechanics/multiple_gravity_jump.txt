GRAVITY_ACCELERATION = 10
-- We consider the jump "peaking" when the speed is between -50 and 50
PEAKING_SPEED = 50
MAX_FALL_VELOCITY = 500
GRAVITY_FALL_MULTIPLIER = 1.5
PEAKING_MULTIPLIER = 0.5
-- ...
-- Are we jumping?
if (speed_y < - PEAKING_SPEED) then
    -- We're rising: apply Gravity Normally
    speed_y = speed_y + GRAVITY_ACCELERATION
elseif (speed_y > PEAKING_SPEED) then
    -- We're falling, enhance gravity
    speed_y = speed_y + GRAVITY_ACCELERATION * GRAVITY_FALL_MULTIPLIER
else
    -- Our jump is peaking, lower gravity to give more time
    speed_y = speed_y + GRAVITY_ACCELERATION * PEAKING_MULTIPLIER
end
-- Cap the fall speed
if (speed_y > MAX_FALL_VELOCITY) then
    speed_y = MAX_FALL_VELOCITY
end
-- ...
