def find_horizontal_matches():
    match_length: int = 0
    min_match_length: int = 3
    for row in matrix:
        last_matching_tile: Tile = None
        for column in row:
            current_tile = matrix[row][column].tile
            if current_tile == last_matching_tile:
                match_length += 1
            else:
                if match_length >= min_match_length:
                    # We need to memorize all the tiles involved in the match
                    for tile in matrix[row][column-match_length:column+1]:
                        memorize(tile)
                else:
                    # No matches, reset the counter and set the current tile as last matching
                    match_length = 1
                    last_matching_tile = current_tile
            # We need to account for the right-hand border corner case
            if column == len(matrix[row]):
                if match_length >= min_match_length:
                    # We need to memorize all the tiles involved in the match
                    for tile in matrix[row][column-match_length:column+1]:
                        memorize(tile)
