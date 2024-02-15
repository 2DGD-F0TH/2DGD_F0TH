#include <list>

class Vector2D;

class Maze{
    private:
        int width;
        int height;
        bool* cells = nullptr;

    public:
        Maze(int w, int h){
            /*
            * A simple constructor
            */
            width = w;  // Needs to be an odd number
            height = h;  // Needs to be an odd number

            cells = new bool[h * w];  // We treat the 2D array as a big 1D array

            // Create a 2D matrix containing the maze data
            // False = Path, True = Wall
            for (int i = 0; i < h; i++) {
                for (int j = 0; j < h; j++) {
                    cells[i * w + j] = true;  // We fill the array with true
                }
            }
        }

        void make_path(int x, int y){
            /*
            * Given a cell coordinates, sets the cell as a path
            */
            cells[y * width + x] = false;
        }

        void make_wall(int x, int y){
            /*
            * Given a cell coordinates, sets the cell as a wall
            */
            cells[y * width + x] = true;
        }

        bool is_wall(int x, int y){
            /*
            * Returns true if the given cell coordinates are inside the maze
            * boundaries and if the selected cell is a wall
            */
            // Let's check if the coordinates are within the maze grid
            if ((x >= 0) && (x < width) && (y >= 0) && (y < height)){
                // if they are, then we can check if the cell is a wall
                return cells[y * width + x];
            }else{
                // If we're outside the maze boundaries, we return false
                return false;
            }
        }

        void draw_maze(int x, int y){
            /*
            * Draws a maze using the recursive backtracker algorithm and
            * system stack recursion.
            *
            * This version will dig walls 2 cells at a time, thus leaving some cells
            * acting as "walls" instead of having a more complex structure.
            */
            // We dig a path in the current cell
            make_path(x, y);
            // We create a list of available directions (x and y)
            std::list<Vector2D> directions = {
                Vector2D(1, 0),
                Vector2D(-1, 0),
                Vector2D(0, 1),
                Vector2D(0, -1)
            };
            // And we shuffle them
            shuffle_in_place(directions);  // We use an external custom-made function for shuffling

            // While there is at least one direction available
            while (!directions.empty()){
                // We take the last item in our directions list (which is random)
                Vector2D direction_to_try = directions.back();
                directions.pop_back();

                // Calculate the new node's coordinates using the chosen direction.
                // We are doubling the movement in each direction, so some cells
                // can act as walls
                int next_x = x + (direction_to_try.x * 2);
                int next_y = y + (direction_to_try.y * 2);

                // If the node we found is a wall, it means we didn't visit it
                // (and it's inside our maze boundaries)
                if (is_wall(next_x, next_y)){
                    // We have found a new node to dig towards

                    // Since we are moving 2 cells at a time, we need to carve
                    // the cell that "links" our "current node" and the "next node"
                    int link_cell_x = x + direction_to_try.x;
                    int link_cell_y = y + direction_to_try.y;
                    make_path(link_cell_x, link_cell_y);

                    // Repeat the carving process with the new coordinates
                    draw_maze(next_x, next_y);
                }
            }
            // If all available directions are exhausted, we return (thus popping
            // one layer from the system stack: we go back one step)
            // If we are back at the starting cell, the algorithm terminates
            return;
        }
};
