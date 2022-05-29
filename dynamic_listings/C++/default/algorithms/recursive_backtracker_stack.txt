#include <list>

class Vector2D;

class Maze{
    private:
        int width;
        int height;
        bool* cells = nullptr;
        std::list<Vector2D>* carved_passages;

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

            carved_passages = new std::list<Vector2D>();
        }

        // ...
        // Same as the previous example
        // ...

        void draw_maze(int x, int y){
            /*
            * Draws a maze using the recursive backtracker algorithm and
            * system stack recursion.
            *
            * This version will dig walls 2 cells at a time, thus leaving some cells
            * acting as "walls" instead of having a more complex structure.
            */
            // First thing, we push our cell into our "carved passages",
            // this will simulate the system stack
            carved_passages->push_back(Vector2D(x,y));
            // When the "carved_passages" array is empty, we are back at the start of the maze
            int stored_x = x;
            int stored_y = y;
            int next_x = -1;  // Magic Value
            int next_y = -1;  // Magic Value
            while (!carved_passages->empty()){
                // We dig a path in the current cell
                make_path(stored_x, stored_y);
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
                    int next_x = stored_x + (direction_to_try.x * 2);
                    int next_y = stored_y + (direction_to_try.y * 2);

                    // If the node we found is a wall, it means we didn't visit it
                    // (and it's inside our maze boundaries)
                    if (is_wall(next_x, next_y)){
                        // We have found a new node to dig towards

                        // Since we are moving 2 cells at a time, we need to carve
                        // the cell that "links" our "current node" and the "next node"
                        int link_cell_x = stored_x + direction_to_try.x;
                        int link_cell_y = stored_y + direction_to_try.y;
                        make_path(link_cell_x, link_cell_y);

                        // Repeat the carving process with the new coordinates
                        // we do this by pushing into the stack
                        carved_passages->push_back(Vector2D(next_x, next_y));
                    }
                }
                // If all available directions are exhausted, we pop
                // one layer from our stack: we go back one step
                if (directions.empty()){
                    Vector2D popped_vector = carved_passages->back();
                    carved_passages->pop_back();
                    stored_x = popped_vector.x;
                    stored_y = popped_vector.y;
                }else{
                    // If not, we continue
                    stored_x = next_x;
                    stored_y = next_y;
                }
            }
        }
};
