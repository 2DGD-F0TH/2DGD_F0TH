const int GRAVITY_ACCELERATION = 10;
const float MAX_FALL_VELOCITY = 500;
// ...
// Apply Gravity
speed_y = speed_y + GRAVITY_ACCELERATION;
// Cap the fall speed
if (speed_y > MAX_FALL_VELOCITY){
    speed_y = MAX_FALL_VELOCITY;
}
// ...
