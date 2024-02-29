class TiledPlayer:
    def __init__(self):
        self.offset = Vector2D(0, 0)
        self.current_position = Vector2D(10, 10)
        self.next_position = Vector2D(10, 10)

    def update(self, dt):
        # ...
        # Check which direction is the player going
        if KEYBOARD.Up_Arrow_Pressed:
            self.offset.y = -1
        if KEYBOARD.Down_Arrow_Pressed:
            self.offset.y = 1
        if KEYBOARD.Right_Arrow_Pressed:
            self.offset.x = 1
        if KEYBOARD.Left_Arrow_Pressed:
            self.offset.x = -1
        # Get the destination tile
        self.next_position = self.current_position + self.offset
        # Is the tile a wall?
        if not MAP.get_tile(self.next_position).isWall():
            # No, move the player to the new tile
            self.current_position = self.next_position
        # ...
