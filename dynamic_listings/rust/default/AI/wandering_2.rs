# //! ```cargo
# //! [dependencies]
# //! rand = "0.10"
# //! ```
#
# #![allow(dead_code, unused)]
#
# #[derive(Clone, PartialEq)]
# struct Cell {
#     x: u32,
#     y: u32,
# }
# impl Cell {
#     fn is_wall(&self) -> bool { todo!() }
# }
# impl Cell {
#     fn new(_: u32, _: u32) -> Self { todo!() }
# }
fn random<T>(vect: &[T]) -> &T {
    // Gets a random item from a Vector
    let x = rand::random_range(0..vect.len());
    &vect[x]
}

// ...

const DIRECTIONS: [&str; 4] = ["NORTH", "EAST", "SOUTH", "WEST"];

struct AiEntity {
    // 0=North, 1=East, ...
    forward_direction_index: usize,
    current_cell: Cell,
}

impl AiEntity {
#   fn move_to(&mut self, _: &Cell) {}
    pub fn new() -> Self {
        Self {
            forward_direction_index: 0,
            current_cell: Cell::new(1, 0),
        }
    }

    pub fn adjacent_cell(cell: &Cell, direction: &str) -> Cell {
        // Returns the adjacent cell in said direction
        let mut cell_copy = cell.clone();
        if direction == "NORTH" {
            cell_copy.y -= 1;
        }
        if direction == "SOUTH" {
            cell_copy.y += 1;
        }
        if direction == "WEST" {
            cell_copy.x -= 1;
        }
        if direction == "EAST" {
            cell_copy.x += 1;
        }
        cell_copy
    }

    fn is_valid(&self, cell: &Cell) -> bool {
        /* Returns true if the cell is valid, aka
         * does not have a wall and does not go backwards */
        if cell.is_wall() {
            // The cell is a wall
            return false;
        }
        if cell
            == &Self::adjacent_cell(
                &self.current_cell,
                DIRECTIONS[(self.forward_direction_index + 2) % 4],
            )
        {
            // We're going backwards, we don't want that
            return false;
        }
        // In all other cases, it's valid
        true
    }

    fn available_directions(&self, cell: &Cell) -> Vec<&'static str> {
        /* Returns a list of available directions */
        let mut result = Vec::new();
        for direction in &DIRECTIONS {
            if self.is_valid(&Self::adjacent_cell(cell, direction)) {
                result.push(*direction);
            }
        }
        result
    }

    fn update(&mut self, dt: f32) {
        // ...
        // Get a list of the available directions
        let available_directions = self.available_directions(&self.current_cell);
        let chosen_direction = if available_directions.is_empty() {
            // No directions are available, let's go back
            &DIRECTIONS[(self.forward_direction_index + 2) % 4]
        } else {
            // Choose a random direction among the available ones
            random(&available_directions)
        };
        // Move
        let next_cell = Self::adjacent_cell(&self.current_cell, chosen_direction);
        self.move_to(&next_cell);
    }
}
