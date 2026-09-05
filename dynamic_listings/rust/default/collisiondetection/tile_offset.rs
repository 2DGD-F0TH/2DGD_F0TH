# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<f32>;
#
# struct Keyboard {
#     left_arrow_pressed: bool,
#     right_arrow_pressed: bool,
#     down_arrow_pressed: bool,
#     up_arrow_pressed: bool,
# }
# static KEYBOARD: Keyboard = Keyboard {
#     left_arrow_pressed: false,
#     right_arrow_pressed: false,
#     down_arrow_pressed: false,
#     up_arrow_pressed: false
# };
# struct Cell;
# impl Cell {
#     fn is_wall(&self) -> bool { todo!() }
# }
# struct Map;
# impl Map {
#     fn tile(&self, _: Vector2D) -> Cell { todo!() }
# }
# static MAP: Map = Map;
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
