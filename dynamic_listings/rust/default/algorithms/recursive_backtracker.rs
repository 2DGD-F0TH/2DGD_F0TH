struct Maze {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        /*
         * A simple constructor
         */
        Self {
            width,                             // Needs to be an odd number
            height,                            // Needs to be an odd number
            cells: vec![true; width * height], // We treat the 2D array as a big 1D array
        }
    }

    pub fn make_path(&mut self, x: usize, y: usize) {
        /*
         * Given a cell coordinates, sets the cell as a path
         */
        self.cells[y * self.width + x] = false;
    }

    pub fn make_wall(&mut self, x: usize, y: usize) {
        /*
         * Given a cell coordinates, sets the cell as a wall
         */
        self.cells[y * self.width + x] = true;
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        /*
         * Returns true if the given cell coordinates are inside the maze
         * boundaries and if the selected cell is a wall
         */
        // Let's check if the coordinates are within the maze grid
        if x < self.width && y < self.height {
            // if they are, then we can check if the cell is a wall
            self.cells[y * self.width + x]
        } else {
            // If we're outside the maze boundaries, we return false
            false
        }
    }

    fn draw_maze(&mut self, x: usize, y: usize) {
        /*
         * Draws a maze using the recursive backtracker algorithm and
         * system stack recursion.
         *
         * This version will dig walls 2 cells at a time, thus leaving some cells
         * acting as "walls" instead of having a more complex structure.
         */
        // We dig a path in the current cell
        self.make_path(x, y);
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
            let next_x = (x as i32 + direction_to_try.x * 2) as usize;
            let next_y = (y as i32 + direction_to_try.y * 2) as usize;

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
                self.draw_maze(next_x, next_y);
            }
        }
        // If all available directions are exhausted, we return (thus popping
        // one layer from the system stack: we go back one step)
        // If we are back at the starting cell, the algorithm terminates
    }
}
