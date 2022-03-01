local function bounce_tween(t)
    -- This constant will allow us to overshoot the max value by around 10%
    C = 1.70158

    return 1 + (C + 1) * (t - 1)^3 + C * (t - 1)^2
end
