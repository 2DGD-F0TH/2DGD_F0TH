void findHorizontalMatches(){
    int matchLength = 0;
    int minMatchLength = 3;
    int rowsize = (sizeof matrix / sizeof matrix[0]);
    for (int row = 0; row < rowsize; ++row){
        Tile* lastMatchingTile = nullptr;
        int colsize = (sizeof matrix[row] / sizeof matrix[row][0]);
        for (int column = 0; column < colsize; ++column){
            Tile* currentTile = matrix[row][column];
            if (currentTile == lastMatchingTile){
                matchLength = matchLength + 1;
            }else{
                if (matchLength >= minMatchLength){
                    // We need to memorize all the tiles involved in the match
                    for (int k = column-matchlength; k < column; ++k) {
                        Tile* tile = matrix[row][k];
                        memorize(tile);
                    }
                }else{
                    // No matches, reset the counter and set the current tile as last matching
                    matchLength = 1;
                    lastMatchingTile = currentTile;
                }
            }
            // We need to account for the right-hand border corner case
            if (column == rowsize){
                if (matchLength >= minMatchLength){
                    // We need to memorize all the tiles involved in the match
                    for (int k = column-matchlength; k < column; ++k) {
                        Tile* tile = matrix[i][k];
                        memorize(tile);
                    }
                }
            }
        }
    }
}
