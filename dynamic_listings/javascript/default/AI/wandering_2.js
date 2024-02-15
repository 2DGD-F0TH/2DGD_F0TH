const DIRECTIONS = ["NORTH", "EAST", "SOUTH", "WEST"];

class AIEntity{
    constructor(){
        // 0=North, 1=East, ...
        this.forward_direction_index = 0;
        this.current_cell = [1,0];
    }
    // ...
    get_adjacent_cell(cell, direction){
        // Returns the adjacent cell in said direction
        let cell_copy = cell.copy();
        if (direction == "NORTH"){
            cell_copy.y -= 1;
        }
        if (direction == "SOUTH"){
            cell_copy.y += 1;
        }
        if (direction == "WEST"){
            cell_copy.x -= 1;
        }
        if (direction == "EAST"){
            cell_copy.x += 1;
        }
        return cell_copy;
    }

    is_valid(cell){
        /* Returns true if the cell is valid, aka
         * does not have a wall and does not go backwards */
        if (cell.is_wall()){
            // The cell is a wall
            return False;
        }
        if (cell == this.get_adjacent_cell(this.current_cell, DIRECTIONS[(forward_direction_index + 2) % 4])){
            // We're going backwards, we don't want that
            return False;
        }
        // In all other cases, it's valid
        return True;
    }

    get_available_directions(cell){
        /* Returns a list of available directions */
        let result = [];
        for (const direction of DIRECTIONS){
            if (is_valid(this.get_adjacent_cell(cell, direction))){
                result.push(direction);
            }
        }
        return result;
    }

    update(dt){
        // ...
        // Get a list of the available directions
        let available_directions = this.get_available_directions(current_cell);
        let chosen_direction = "NORTH";  // Just a default
        if (available_directions.length === 0){
            // No directions are available, let's go back
            chosen_direction = DIRECTIONS[(forward_direction_index + 2) % 4];
        }else{
            // Choose a random direction among the available ones
            chosen_direction = available_directions[Math.random() * available_directions.length];
        }
        // Move
        next_cell = this.get_adjacent_cell(current_cell, chosen_direction);
        this.move_to(next_cell);
    }
}
