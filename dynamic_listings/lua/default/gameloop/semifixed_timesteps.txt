local dt = 1.0/60.0
local game_is_running = true

-- We bootstrap frametime for 1/60th of a second for the first frame
local frametime = 1.0/60.0

while (game_is_running) do
    -- We get the system time in milliseconds
    -- since implementation varies here i'll use a generic function name
    local begin = get_system_time_millis()

    while (frametime > 0.0) do
        local deltaTime = min(dt, frametime)
        process_user_input()
        update_world(deltaTime)
        frametime = frametime - deltaTime
    end
    draw()
    local end = get_system_time_millis();
    -- We memorize how long this frame lasted
    frametime = end - begin
end
