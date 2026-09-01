struct TiledPlayer {
    offset: Vector2D,
    current_position: Vector2D,
    next_position: Vector2D,
}

impl TiledPlayer {
    pub fn new() -> Self {
        Self {
            offset: Vector2D::new(0., 0.),
            current_position: Vector2D::new(10., 10.),
            next_position: Vector2D::new(10., 10.),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // ...
        // Check which direction is the player going
        if KEYBOARD.up_arrow_pressed {
            self.offset.y = -1.;
        }
        if KEYBOARD.down_arrow_pressed {
            self.offset.y = 1.;
        }
        if KEYBOARD.right_arrow_pressed {
            self.offset.x = 1.;
        }
        if KEYBOARD.left_arrow_pressed {
            self.offset.x = -1.;
        }
        // Get the destination tile
        self.next_position = self.current_position + self.offset;
        // Is the tile a wall?
        if !MAP.tile(self.next_position).is_wall() {
            // No, move the player to the new tile
            self.current_position = self.next_position;
        }
        // ...
    }
}
