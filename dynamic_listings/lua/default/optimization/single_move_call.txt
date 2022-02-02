function update(dt)
    local vector_up = (0, -1)
    local vector_right = (1, 0)
    -- ...
    local total_movement = vector_up + vector_right
    characterController.Move(total_movement * dt)
    -- ...
end
