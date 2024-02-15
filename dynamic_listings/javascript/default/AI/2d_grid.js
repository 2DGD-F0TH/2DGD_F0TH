class Grid_2D{
    // Represents a 2D grid of "Tile" classes

    constructor(rows, cols){
        // Prepares the memory for the grid
        this.grid = Array.from(Array(rows), () => new Array(cols));
        this.height = rows;
        this.width = cols;
    }

    getCell(row, col){
        // Gets a cell from the 2D Grid
        if ((row >= 0 && row < height) && (col >=0 && col < width)){
            // We better check if we are inside the grid
            return grid[row][col];
        }else{
            return null;
        }
    }

    getAdjacentCells(row, col){
        /* Returns a list of cells adjacent the ones we give
        REMEMBER: We index at 0 so the first row is 0, the last one is at
        "height - 1", same goes for columns */
        let toReturn = [];
        if ((row >= 0 && row < height) && (col >=0 && col < width)){
            // We better check if we are inside the grid
            if (row > 0){
                // We are not on the first row, we can add the cell above
                toReturn.push(this.getCell(row - 1, col));
            }
            if (row < height - 1){
                // We are not on the last row, we can add the cell below
                toReturn.push(this.getCell(row + 1, col));
            }
            if (col > 0){
                // We are not on the first column, we can add the cell on the left
                toReturn.push(this.getCell(row, col - 1));
            }
            if (col < width - 1){
                // We are not on the last column, we can add the cell on the right
                toReturn.push(this.getCell(row, col + 1));
            }
        }
        /* If the checks went well, toReturn will have
        a list of the adjacent cells, if not it will be empty */
        return toReturn;
    }
}
