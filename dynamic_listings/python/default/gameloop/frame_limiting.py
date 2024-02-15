import time

targetTime = 1.0/60.0
game_is_running = True

# We bootstrap dt to 1/60th of a second for the first frame
dt = 1.0/60.0

while game_is_running:
    # We get the system time in milliseconds
    begin = int(time.time() * 1000)
    process_user_input()
    update_world(dt)
    draw()
    end = int(time.time() * 1000)
    # We update our dt
    dt = end - begin
    # If some time is left, we wait until we reach 1/60th of a second.
    time.sleep(max(targetTime - dt, 0))
