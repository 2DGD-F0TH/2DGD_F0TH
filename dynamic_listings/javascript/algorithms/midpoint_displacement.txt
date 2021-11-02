let MIN = 0;
let MAX = 100;
let OCTAVES = 5;

// This will contain the "heights" of our terrain
let terrain = [];

// We start by deciding the start and end "heights" of our terrain
terrain[0] = Math.random() * (MAX - MIN) + MIN;
terrain[31] = Math.random() * (MAX - MIN) + MIN;
// We interpolate all the missing values
interpolate(terrain, 0, 31);


function midpoint_displacement(begin, end, octave){
    // Get the midpoint
    let midpoint = Math.floor((end - begin) / 2);
    // Get the midpoint value
    let value = (Math.abs(terrain[end] - terrain[begin])) / 2;
    // Get the possible displacement
    let displacement = MAX / octave;
    // Displace by a random amount
    value = value + Math.random() * 2 * displacement - displacement;
    // Apply the value
    terrain[midpoint] = value;
    // Interpolate the values between begin and midpoint
    for (let i = begin + 1; i < midpoint;  i++){
        terrain[i] = interpolate(terrain, begin, midpoint);
    }
    // Interpolate the values between midpoint and the end
    for (let i = midpoint + 1; i < end;  i++){
        terrain[i] = interpolate(terrain, midpoint, end);
    }
    // Recursion on the subtree
    if (octave < OCTAVES){
        // Recur left
        midpoint_displacement(begin, midpoint, octave + 1);
        // Recur right
        midpoint_displacement(midpoint, end, octave + 1);
    }
}
