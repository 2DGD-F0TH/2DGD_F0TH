use std::collections::BTreeMap;

fn create_new_tiles() -> BTreeMap<Tile, i32> {
    // Our falling tiles list, will be used for tweening
    let mut falling_tiles = BTreeMap::new();
    // We scroll each column of the board
    for column in &mut matrix {
        for tile in column {
            if tile.is_none() {
                let mut new_tile = Tile::create_random();
                new_tile.y = -64; // A value that is out of the board
                *tile = Some(new_tile.clone());

                // Add this tile to the falling tiles mapping
                falling_tiles.insert(new_tile.clone(), new_tile.y);
            }
        }
    }
    falling_tiles
}
