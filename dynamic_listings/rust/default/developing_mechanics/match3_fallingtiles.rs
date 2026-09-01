use std::collections::BTreeMap;

fn remove_matches(&mut self, matches: &mut Vec<Tile>) {
    while let Some(tile) = matches.pop() {
        self.matrix[tile.y][tile.x] = None;
    }
}

fn find_falling_tiles(&mut self) -> BTreeMap<Tile, usize> {
    // Our falling tiles list, will be used for tweening
    let mut falling_tiles = BTreeMap::new();
    // We scroll each column of the board
    let colsize = self.matrix.len();
    for x in 0..colsize {
        let mut found_space = false;
        let mut space_y = 0;
        // We scroll each row, from bottom to top
        let mut y = self.matrix[x].len();
        while y > 0 {
            let tile = self.matrix[y][x];
            if found_space {
                // If the current tile is not a space, bring it to the lowest space
                if let Some(mut t) = tile {
                    // Put it in the correct spot
                    self.matrix[space_y][x] = tile;
                    t.y = space_y;

                    // Set the old position to empty
                    self.matrix[x][y] = None;

                    // Set the tween starting position for later
                    falling_tiles.insert(t, y);

                    // We reset the found_space for next loop
                    found_space = false;
                    // We need to re-scan this tile (it will be empty, but there may be more tiles above)
                    y = space_y;

                    // Reset spaceY for next loop
                    space_y = 0;
                }
            } else if tile.is_none() {
                found_space = true;
                // In case we didn't find a space yet, this is the one
                if space_y == 0 {
                    space_y = y;
                }
            }
            // We go up one tile
            y -= 1;
        }
    }
    falling_tiles
}
