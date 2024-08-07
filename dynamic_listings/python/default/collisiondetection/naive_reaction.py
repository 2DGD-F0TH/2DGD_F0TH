# Naive collision reaction with rectangles
def update(dt: float) -> None:
    # ...
    player.position += player.speed * dt
    # Refer to your favourite collision detection and broad/fine passes
    if collision(player, object):
        if player.x_speed > 0:  # going right
            player.position.x = object.rectangle.left  # reset position
            player.x_speed = 0  # stop the player
        if player.x_speed < 0:  # going left
            player.position.x = object.rectangle.right # reset position
            player.x_speed = 0  # stop the player
    # Again, refer to your favourite collision detection and broad/fine passes
    if collision(player, object):
        if player.y_speed > 0:  # going down
            player.position.y = object.rectangle.top  # reset position
            player.y_speed = 0  # stop the player
        if player.y_speed > 0:  # going up
            player.position.y = object.rectangle.bottom  # reset position
            player.y_speed = 0  # stop the player
    # ...
