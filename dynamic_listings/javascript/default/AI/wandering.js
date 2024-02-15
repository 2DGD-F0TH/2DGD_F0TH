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

    update(dt){
        // ...
        // Choose a random direction
        let chosen_direction = DIRECTIONS[Math.random() * DIRECTIONS.length];
        let i = 0;
        let next_cell = this.get_adjacent_cell(current_cell, chosen_direction);
        while (!this.is_valid(next_cell) && i != 4){
            chosen_direction = DIRECTIONS[(DIRECTIONS.indexOf(chosen_direction) + 1) % 4];
            next_cell = this.get_adjacent_cell(current_cell, chosen_direction);
            i = i + 1;
        }
        if (i == 4){
            // We exhausted the possibilities, go backwards
            chosen_direction = DIRECTIONS[(forward_direction_index + 2) % 4];
            next_cell = this.get_adjacent_cell(current_cell, chosen_direction);
        }
        // Move
        this.move_to(next_cell);
    }
}
