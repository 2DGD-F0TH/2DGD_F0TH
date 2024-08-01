INVINCIBILITY_TIME: float = 0.75  # Seconds of invincibility
...
def update(dt) -> None:
    inv_time: float = 0
    ...
    if inv_time <= 0:
        # Check for collision
        ...
        # Collision has been detected here, we have a hit
        inv_time = INVINCIBILITY_TIME  # Start of the invincibility period
    else:
        # We are currently invincible
        inv_time = inv_time - dt  # We decrease the invincibility time
