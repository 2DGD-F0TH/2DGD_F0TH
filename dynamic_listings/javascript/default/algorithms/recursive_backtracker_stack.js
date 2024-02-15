class Maze{
    constructor(w, h){
        /*
         * A simple constructor
         */
        this.width = w;  // Needs to be an odd number
        this.height = h;  // Needs to be an odd number

        // Create a 2D matrix containing the maze data
        // False = Path, True = Wall
        this.cells = [];
        // Create a 2D array filled with "True"
        for (let i = 0; i < h; i++){
            this.cells[i] = new Array(w).fill(true);
        }
        this.carved_passages = [];
    }

    // ...
    // Same as the previous example
    // ...

    draw_maze(x, y){
        /*
         * Draws a maze using the recursive backtracker algorithm and
         * system stack recursion.
         *
         * This version will dig walls 2 cells at a time, thus leaving some cells
         * acting as "walls" instead of having a more complex structure.
        */
        // First thing, we push our cell into our "carved passages",
        // this will simulate the system stack
        this.carved_passages.push({"x": x, "y": y});
        // When the "carved_passages" array is empty, we are back at the start of the maze
        let stored_x = x;
        let stored_y = y;
        let next_x = null;
        let next_y = null;
        // While the array is not empty
        while (!carved_passages.length != 0){
            // We dig a path in the current cell
            make_path(stored_x, stored_y);
            // We create a list of available directions (x and y)
            directions = [
                {"x": 1, "y": 0},
                {"x": -1, "y": 0},
                {"x": 0, "y": 1},
                {"x": 0, "y": -1}
            ];
            // And we shuffle them (it needs a specialized function, cause javascript doesn't have one)
            directions = shuffle_array(directions);

            // While there is at least one direction available
            while (directions.length != 0){
                // We take the last item in our directions list (which is random)
                direction_to_try = directions.pop();

                // Calculate the new node's coordinates using the chosen direction.
                // We are doubling the movement in each direction, so some cells
                // can act as walls
                next_x = stored_x + (direction_to_try.x * 2);
                next_y = stored_y + (direction_to_try.y * 2);

                // If the node we found is a wall, it means we didn't visit it
                // (and it's inside our maze boundaries)
                if (is_wall(next_x, next_y)){
                    // We have found a new node to dig towards

                    // Since we are moving 2 cells at a time, we need to carve
                    // the cell that "links" our "current node" and the "next node"
                    let link_cell_x = stored_x + direction_to_try.x;
                    let link_cell_y = stored_y + direction_to_try.y;
                    make_path(link_cell_x, link_cell_y);

                    // Repeat the carving process with the new coordinates
                    // we do this by pushing into the stack
                    carved_passages.push({"x": next_x, "y": next_y});
                }
            }
            // If all available directions are exhausted, we pop
            // one layer from our stack: we go back one step
            if (directions.length == 0){
                let popped_vector = carved_passages.pop();
                stored_x = popped_vector.x;
                stored_y = popped_vector.y;
            }else{
                // If not, we continue
                stored_x = next_x;
                stored_y = next_y;
            }
        }
    }
}
