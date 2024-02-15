function game()
    local game_is_running = true
    while (game_is_running) do
        process_user_input()
        update_world()
        draw()
    end
end
