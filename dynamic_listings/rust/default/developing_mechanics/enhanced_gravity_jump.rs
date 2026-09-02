# #![allow(unused)]
# fn main() {
# let mut speed_y = 0.;
const GRAVITY_ACCELERATION: f32 = 10.;
const MAX_FALL_VELOCITY: f32 = 500.;
const GRAVITY_FALL_MULTIPLIER: f32 = 1.5;
// ...
// Are we jumping?
if speed_y < 0. {
    // Apply Gravity Normally
    speed_y = speed_y + GRAVITY_ACCELERATION;
} else {
    // We're falling, enhance gravity
    speed_y = speed_y + GRAVITY_ACCELERATION * GRAVITY_FALL_MULTIPLIER;
}
// Cap the fall speed
if speed_y > MAX_FALL_VELOCITY {
    speed_y = MAX_FALL_VELOCITY;
}
// ...
# }
