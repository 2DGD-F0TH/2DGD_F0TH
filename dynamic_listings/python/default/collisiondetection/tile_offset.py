class TiledPlayer:
    def __init__(self) -> None:
        self.offset: Vector2 = Vector2(0, 0)
        self.current_position: Vector2 = Vector2(10, 10)
        self.next_position: Vector2 = Vector2(10, 10)

    def update(self, dt: float) -> None:
        # ...
        # Check which direction is the player going
        if KEYBOARD.is_up_arrow_pressed:
            self.offset.y = -1
        if KEYBOARD.is_down_arrow_pressed:
            self.offset.y = 1
        if KEYBOARD.is_right_arrow_pressed:
            self.offset.x = 1
        if KEYBOARD.is_left_arrow_pressed:
            self.offset.x = -1
        # Get the destination tile
        self.next_position = self.current_position + self.offset
        # Is the tile a wall?
        if not MAP.get_tile(self.next_position).is_wall():
            # No, move the player to the new tile
            self.current_position = self.next_position
        # ...
