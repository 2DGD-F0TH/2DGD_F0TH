# #![allow(dead_code)]
# fn memorize(_: f32) {}
fn find_horizontal_matches() {
# let matrix = [[0.; 1]; 1];
    let mut match_length = 0;
    let min_match_length = 3;
    let rowsize = matrix.len();
    for row in 0..rowsize {
        let mut last_matching_tile = None;
        let colsize = matrix[row].len();
        for column in 0..colsize {
            let current_tile = matrix[row][column];
            if Some(current_tile) == last_matching_tile {
                match_length += 1;
            } else {
                if match_length >= min_match_length {
                    // We need to memorize all the tiles involved in the match
                    for k in column - match_length..column {
                        let tile = matrix[row][k];
                        memorize(tile);
                    }
                } else {
                    // No matches, reset the counter and set the current tile as last matching
                    match_length = 1;
                    last_matching_tile = Some(current_tile);
                }
            }
            // We need to account for the right-hand border corner case
            if column == rowsize {
                if match_length >= min_match_length {
                    // We need to memorize all the tiles involved in the match
                    for k in column - match_length..column {
                        let tile = matrix[row][k];
                        memorize(tile);
                    }
                }
            }
        }
    }
}
