local function point_triangle_collision(px, py, x1, y1, x2, y2, x3, y3)
    -- We accept anything that is closer than 1/1000th of unit
    local epsilon = 0.0001
    local original_area = math.abs((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1))
    local area1 = math.abs((x1-px)*(y2-py) - (x2-px)*(y1-py))
    local area2 = math.abs((x2-px)*(y3-py) - (x3-px)*(y2-py))
    local area3 = math.abs((x3-px)*(y1-py) - (x1-px)*(y3-py))
    if (math.abs(area1 + area2 + area3 - original_area) < epsilon) then
        return true
    else
        return false
    end
end
