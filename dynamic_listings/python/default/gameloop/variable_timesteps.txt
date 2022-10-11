import time

game_is_running = True

# We initialize our dt at 1/60th of a second for the first loop
dt = 1.0/60.0;

while game_is_running:
    # We get the system time in milliseconds
    begin = int(time.time() * 1000)
    process_user_input()
    update_world(dt)
    draw()
    end = int(time.time() * 1000)
    # We update our Dt
    dt = end - begin
