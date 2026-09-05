# //! ```cargo
# //! [dependencies]
# //! rand = "0.10"
# //! ```
#
# #![allow(dead_code, unused)]
#
# fn interpolate(_: &[f32], _: usize, _: usize) -> f32 { todo!() }
# fn main() {
const MIN: f32 = 0.;
const MAX: f32 = 100.;
const OCTAVES: f32 = 5.;

let mut terrain = [0.; 32];

terrain[0] = rand::random_range(MIN..=MAX);
terrain[31] = rand::random_range(MIN..=MAX);
interpolate(&terrain, 0, 31);

fn midpoint_displacement(begin: usize, end: usize, octave: f32) {
#   let mut terrain = [0_f32; 32];
    // Get the midpoint
    let midpoint = (end - begin) / 2;
    // Get the midpoint value
    let mut value = (terrain[end] - terrain[begin]).abs() / 2.;
    // Get the possible displacement
    let displacement = MAX / octave;
    // Displace by a random amount
    value += rand::random::<f32>() % (2. * displacement) - displacement;
    // Apply the value
    terrain[midpoint] = value;
    // Interpolate the values between begin and midpoint
    for x in begin + 1..midpoint {
        terrain[x] = interpolate(&terrain, begin, midpoint);
    }
    // Interpolate the values between midpoint and the end
    for x in midpoint + 1..end {
        terrain[x] = interpolate(&terrain, midpoint, end);
    }
    // Recursion on the subtree
    if octave < OCTAVES {
        // Recur left
        midpoint_displacement(begin, midpoint, octave + 1.);
        // Recur right
        midpoint_displacement(midpoint, end, octave + 1.);
    }
}
# }
