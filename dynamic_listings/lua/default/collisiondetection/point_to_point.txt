local function point_collision(A, B)
    if (A.x == B.x and A.y == B.y) then
        return true
    else
        return false
    end
end
