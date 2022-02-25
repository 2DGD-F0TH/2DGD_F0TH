function update(dt)
    local vector_up = {0, -1}
    local vector_right = {1, 0}
    -- ...
    characterController.Move(vector_up * dt)
    characterController.Move(vector_right * dt)
    -- ...
end
