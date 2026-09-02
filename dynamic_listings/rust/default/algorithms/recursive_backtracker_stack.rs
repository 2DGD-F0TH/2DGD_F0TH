# //! ```cargo
# //! [dependencies]
# //! vector2d = "3.1"
# //! ```
#
# #![allow(dead_code, unused)]
#
# type Vector2D = vector2d::Vector2D<i32>;
# fn shuffle_in_place(_: &mut Vec<Vector2D>) {}
struct Maze {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    carved_passages: Vec<Vector2D>,
}

impl Maze {
#   fn make_path(&mut self, x: usize, y: usize) {}
#   fn is_wall(&self, x: usize, y: usize) -> bool { todo!() }
    pub fn new(width: usize, height: usize) -> Self {
        /*
         * A simple constructor
         */
        Self {
            width,                             // Needs to be an odd number
            height,                            // Needs to be an odd number
            cells: vec![true; width * height], // We treat the 2D array as a big 1D array
            carved_passages: Vec::new(),
        }
    }

    // ...
    // Same as the previous example
    // ...

    fn draw_maze(&mut self, x: usize, y: usize) {
        /*
         * Draws a maze using the recursive backtracker algorithm and
         * system stack recursion.
         *
         * This version will dig walls 2 cells at a time, thus leaving some cells
         * acting as "walls" instead of having a more complex structure.
         */
        // First thing, we push our cell into our "carved passages",
        // this will simulate the system stack
        self.carved_passages.push(Vector2D::new(x as i32, y as i32));
        // When the "carved_passages" array is empty, we are back at the start of the maze
        let mut stored_x = x;
        let mut stored_y = y;
        let mut next_x = 0;
        let mut next_y = 0;
        while !self.carved_passages.is_empty() {
            // We dig a path in the current cell
            self.make_path(stored_x, stored_y);
            // We create a list of available directions (x and y)
            let mut directions = vec![
                Vector2D::new(1, 0),
                Vector2D::new(-1, 0),
                Vector2D::new(0, 1),
                Vector2D::new(0, -1),
            ];
            // And we shuffle them
            shuffle_in_place(&mut directions); // We use an external custom-made function for shuffling

            // While there is at least one direction available
            // We take the last item in our directions list (which is random)
            while let Some(direction_to_try) = directions.pop() {
                // Calculate the new node's coordinates using the chosen direction.
                // We are doubling the movement in each direction, so some cells
                // can act as walls
                next_x = (stored_x as i32 + direction_to_try.x * 2) as usize;
                next_y = (stored_y as i32 + direction_to_try.y * 2) as usize;

                // If the node we found is a wall, it means we didn't visit it
                // (and it's inside our maze boundaries)
                if self.is_wall(next_x, next_y) {
                    // We have found a new node to dig towards

                    // Since we are moving 2 cells at a time, we need to carve
                    // the cell that "links" our "current node" and the "next node"
                    let link_cell_x = (x as i32 + direction_to_try.x) as usize;
                    let link_cell_y = (y as i32 + direction_to_try.y) as usize;
                    self.make_path(link_cell_x, link_cell_y);

                    // Repeat the carving process with the new coordinates
                    // we do this by pushing into the stack
                    self.carved_passages
                        .push(Vector2D::new(next_x as i32, next_y as i32));
                }
            }
            // If all available directions are exhausted, we pop
            // one layer from our stack: we go back one step
            if directions.is_empty() {
                let Some(popped_vector) = self.carved_passages.pop() else {
                    continue;
                };
                stored_x = popped_vector.x as usize;
                stored_y = popped_vector.y as usize;
            } else {
                // If not, we continue
                stored_x = next_x;
                stored_y = next_y;
            }
        }
    }
}
