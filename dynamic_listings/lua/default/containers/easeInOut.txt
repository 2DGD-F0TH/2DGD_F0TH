local function easeInOut(time, duration, power)
    local threshold = duration / 2
    if (time <= threshold) then
        return easeIn(time, duration, power)
    else
        return easeOut(time,duration, power)
    end
end
