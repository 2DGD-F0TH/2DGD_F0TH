local function clamp(value, min, max)
    -- Clamps "value" so it is always between "min" and "max"
    if (value < min) then
        return min
    end
    if (value > max) then
        return max
    end
    return value
end
