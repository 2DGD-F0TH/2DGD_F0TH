def find_vertical_matches():
    match_length = 0
    min_match_length = 3
    for column in matrix:
        last_matching_tile = None
        for row in column:
            current_tile = matrix[row][column].tile
            if current_tile == last_matching_tile:
                match_length += 1
            else:
                if match_length >= min_match_length:
                    # We need to memorize all the tiles involved in the match
                    for r in range(row-match_length, column+1):
                        tile = matrix[r][column]
                        memorize(tile)
                else:
                    # No matches, reset the counter and set the current tile as last matching
                    match_length = 1
                    last_matching_tile = current_tile
            # We need to account for the bottom border corner case
            if row == len(matrix[column]):
                if match_length >= min_match_length:
                    # We need to memorize all the tiles involved in the match
                    for r in range(row-match_length, column+1):
                        tile = matrix[r][column]
                        memorize(tile)
