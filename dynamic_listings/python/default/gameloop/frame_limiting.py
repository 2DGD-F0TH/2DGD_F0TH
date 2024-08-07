import time

TARGET_TIME: float = 1.0/60.0
game_is_running: bool = True

# We bootstrap dt to 1/60th of a second for the first frame
dt: float = 1.0/60.0

while game_is_running:
    # We get the system time in milliseconds
    begin: float = time.time() * 1000
    process_user_input()
    update_world(dt)
    draw()
    end: float = time.time() * 1000
    # We update our dt
    dt = end - begin
    # If some time is left, we wait until we reach 1/60th of a second.
    time.sleep(max(TARGET_TIME - dt, 0))
