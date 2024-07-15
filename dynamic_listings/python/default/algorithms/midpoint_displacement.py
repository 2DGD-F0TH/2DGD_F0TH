from random import uniform as random_float
from math import floor

MIN: int = 0
MAX: int = 100
OCTAVES: int = 5

# This will contain the "heights" of our terrain
terrain: list[float] = [0.0] * 32

# We start by deciding the start and end "heights" of our terrain
terrain[0] = random_float(MIN, MAX)
terrain[31] = random_float(MIN, MAX)
# We interpolate all the missing values
interpolate(terrain, 0, 31)


def midpoint_displacement(begin: int, end: int, octave: int) -> None:
    # Get the midpoint
    midpoint: int = floor((end - begin) / 2)
    # Get the midpoint value
    value: float = (abs(terrain[end] - terrain[begin])) / 2
    # Get the possible displacement
    displacement: float = MAX / octave
    # Displace by a random amount
    value += random_float(-displacement, displacement)
    # Apply the value
    terrain[midpoint] = value
    # Interpolate the values between begin and midpoint
    for i in range(begin + 1, midpoint - 1):
        value[i] = interpolate(terrain, begin, midpoint)
    # Interpolate the values between midpoint and the end
    for i in range(midpoint + 1, end - 1):
        value[i] = interpolate(terrain, midpoint, end)
    # Recursion on the subtree
    if octave < OCTAVES:
        # Recur left
        midpoint_displacement(begin, midpoint, octave + 1)
        # Recur right
        midpoint_displacement(midpoint, end, octave + 1)
