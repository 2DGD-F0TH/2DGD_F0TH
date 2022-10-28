function removeMatches(){
    for (const tile in matches){
        matrix[tile.y][tile.x] = null;
    }
    matches = [];
}

function findFallingTiles(){
    // Our falling tiles list, will be used for tweening
    let falling_tiles = {};
    // We scroll each column of the board
    for (constcolumn in matrix){
        let found_space = false;
        let spaceY = 0;
        // We scroll each row, from bottom to top
        let y = matrix.length();
        while (y > 0){
            let tile = matrix[y][x];
            if (found_space){
                // If the current tile is not a space, bring it to the lowest space
                if (tile != null){
                    // Put it in the correct spot
                    matrix[spaceY][column] = tile;
                    tile.y = spaceY;

                    // Set the old position to empty
                    matrix[column][y] = null;

                    // Set the tween starting position for later
                    falling_tiles[tile] = y;

                    // We reset the found_space for next loop
                    found_space = false;
                    // We need to re-scan this tile (it will be empty, but there may be more tiles above)
                    y = spaceY;

                    // Reset spaceY for next loop
                    spaceY = 0;
                }
            }else if (tile == null){
                found_space = true
                // In case we didn't find a space yet, this is the one
                if (spaceY == 0){
                    spaceY = y;
                }
            }
            // We go up one tile
            y = y - 1;
        }
    }
    return falling_tiles;
}
