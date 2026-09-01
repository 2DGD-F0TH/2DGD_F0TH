fn find_vertical_matches() {
    let mut match_length = 0;
    let min_match_length = 3;
    let colsize = matrix[0].len();
    for column in 0..colsize {
        let mut last_matching_tile = None;
        let rowsize = matrix.len();
        for row in 0..rowsize {
            let current_tile = matrix[row][column];
            if Some(current_tile) == last_matching_tile {
                match_length += 1;
            } else {
                if match_length >= min_match_length {
                    // We need to memorize all the tiles involved in the match
                    for k in row - match_length..row {
                        let tile = matrix[k][column];
                        memorize(tile);
                    }
                } else {
                    // No matches, reset the counter and set the current tile as last matching
                    match_length = 1;
                    last_matching_tile = Some(current_tile);
                }
            }
            // We need to account for the right-hand border corner case
            if row == colsize {
                if match_length >= min_match_length {
                    // We need to memorize all the tiles involved in the match
                    for k in row - match_length..row {
                        let tile = matrix[k][column];
                        memorize(tile);
                    }
                }
            }
        }
    }
}
