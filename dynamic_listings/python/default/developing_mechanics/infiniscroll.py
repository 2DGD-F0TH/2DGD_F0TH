BACKGROUND_X_SIZE: int = 512  # The horizontal size of the background
LOOP_POINT: int = 256  # The horizontal loop point of the image
DISTANCE_FACTOR: float = 0.5  # The background moves at half the player speed


background_x_offset: float = 0.0  # The x offset of the background

def update(dt: float) -> None:
    ...
    # In case we're moving right, the background scrolls left slightly
    if player.speed_x > 0:
        # Update player's position and state
        ...
        # Update the background position
        background_x_offset = background_x_offset - player.speed_x * DISTANCE_FACTOR * dt
        # If we passed the image's loop point
        if background_x_offset <= - LOOP_POINT:
            # We reset the coordinates, keeping note of the remainder
            background_x_offset = background_x_offset % LOOP_POINT
    # In case we're moving left, the background scrolls right slightly
    if player.speed_x < 0:
        # Update player's position and state
        ...
        # Update the background position
        background_x_offset = background_x_offset - player.speed_x * DISTANCE_FACTOR * dt
        if background_x_offset >= 0:
            # We reset the coordinates, keeping note of the remainder, just backwards
            background_x_offset = background_x_offset - BACKGROUND_X_SIZE

def draw():
    ...
    # Draw the background
    screen.draw(background, (background_x_offset, 0))
    ...
