const GRAVITY_ACCELERATION = 10;
// We consider the jump "peaking" when the speed is between -50 and 50
const PEAKING_SPEED = 50
const MAX_FALL_VELOCITY = 500;
const GRAVITY_FALL_MULTIPLIER = 1.5;
const PEAKING_MULTIPLIER = 0.5;
// ...
// Are we jumping?
if (speed_y < - PEAKING_SPEED){
    // We're rising: apply Gravity Normally
    speed_y = speed_y + GRAVITY_ACCELERATION;
}else if (speed_y > PEAKING_SPEED){
    // We're falling, enhance gravity
    speed_y = speed_y + GRAVITY_ACCELERATION * GRAVITY_FALL_MULTIPLIER;
}else{
    // Our jump is peaking, lower gravity to give more time
    speed_y = speed_y + GRAVITY_ACCELERATION * PEAKING_MULTIPLIER;
}
// Cap the fall speed
if (speed_y > MAX_FALL_VELOCITY){
    speed_y = MAX_FALL_VELOCITY;
}
// ...
