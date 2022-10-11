local game_is_running = true

-- We initialize our dt at 1/60th of a second for the first loop
local dt = 1.0/60.0

while (game_is_running) do
    -- We get the system time in milliseconds
    -- since implementation varies here i'll use a generic function name
    local begin = get_system_time_millis()
    process_user_input()
    update_world(dt)
    draw()
    local end = get_system_time_millis()
    -- We update our dt
    dt = end - begin
end
