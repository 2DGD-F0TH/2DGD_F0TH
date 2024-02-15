local dt = 1.0/60.0
local game_is_running = true

while (game_is_running) do
    process_user_input()
    update_world(dt)
    draw()
end
