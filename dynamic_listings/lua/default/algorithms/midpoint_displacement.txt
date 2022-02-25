MIN = 0
MAX = 100
OCTAVES = 5

-- This will contain the "heights" of our Terrain
Terrain = {}

-- We seed the random number generator
math.randomseed(os.time())

-- We start by deciding the start and end "heights" of our Terrain
Terrain[0] = math.random(MIN, MAX - 1) + math.random()
Terrain[31] = math.random(MIN, MAX - 1) + math.random()
-- We interpolate all the missing values
interpolate(Terrain, 0, 31)


local function midpoint_displacement(start, stop, octave)
    -- Get the midpoint
    local midpoint = math.floor((stop - start) / 2)
    -- Get the midpoint value
    local value = (math.abs(Terrain[stop] - Terrain[start])) / 2
    -- Get the possible displacement
    local displacement = MAX / octave
    -- Displace by a random amount
    value = value + math.random(-displacement, displacement - 1) + math.random()
    -- Apply the value
    Terrain[midpoint] = value
    -- Interpolate the values between start and midpoint
    for i = start + 1, midpoint - 1 do
        Terrain[i] = interpolate(Terrain, start, midpoint)
    end
    -- Interpolate the values between midpoint and the stop
    for i = start + 1, midpoint - 1 do
        Terrain[i] = interpolate(Terrain, midpoint, stop)
    end
    -- Recursion on the subtree
    if octave < OCTAVES then
        -- Recur left
        midpoint_displacement(start, midpoint, octave + 1)
        -- Recur right
        midpoint_displacement(midpoint, stop, octave + 1)
    end
end
