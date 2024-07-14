import time

dt = 1.0/60.0
game_is_running = True


# We bootstrap frametime for 1/60th of a second for the first frame
frame_time = 1.0/60.0

while game_is_running:
    # We get the system time in milliseconds
    begin = int(time.time() * 1000)

    while frame_time > 0.0:
        delta_time = min(dt, frame_time)
        process_user_input()
        update_world(dt)
        frame_time = frame_time - delta_time
    draw()
    end = int(time.time() * 1000)
    # We memorize how long this frame lasted
    frame_time = end - begin
