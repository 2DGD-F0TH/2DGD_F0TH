import time

dt = 1.0/60.0
game_is_running = True


# We bootstrap frametime for 1/60th of a second for the first frame
frametime = 1.0/60.0

while game_is_running:
    # We get the system time in milliseconds
    begin = int(time.time() * 1000)

    while frametime > 0.0:
        deltaTime = min(dt, frametime)
        process_user_input()
        update_world(dt)
        frametime = frametime - deltaTime
    draw()
    end = int(time.time() * 1000)
    # We memorize how long this frame lasted
    frametime = end - begin
