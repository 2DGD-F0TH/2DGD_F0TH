# #![allow(unused)]
# fn main() {
# let mut speed_y = 0.;
const GRAVITY_ACCELERATION: f32 = 10.;
const MAX_FALL_VELOCITY: f32 = 500.;
// ...
// Apply Gravity
speed_y += GRAVITY_ACCELERATION;
// Cap the fall speed
if speed_y > MAX_FALL_VELOCITY {
    speed_y = MAX_FALL_VELOCITY;
}
// ...
# }
