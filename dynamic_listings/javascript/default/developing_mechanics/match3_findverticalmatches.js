function findVerticalMatches(){
    let matchLength = 0;
    const minMatchLength = 3;
    matches = [];
    for (const column in matrix){
        let lastMatchingTile = null;
        for (const row in column){
            let currentTile = matrix[row][column].tile;
            if (currentTile == lastMatchingTile){
                matchLength = matchLength + 1;
            }else{
                if (matchLength >= minMatchLength){
                    // We need to memorize all the tiles involved in the match
                    for (let i = row-matchLength; i <= row; i++){
                        matches.push(matrix[i][column]);
                    }
                }else{
                    // No matches, reset the counter and set the current tile as last matching
                    matchLength = 1;
                    lastMatchingTile = currentTile;
                }
            }
            // We need to account for the bottom border corner case
            if (row == matrix.length){
                if (matchLength >= minMatchLength){
                    // We need to memorize all the tiles involved in the match
                    for (let i = row-matchLength; i <= row; i++){
                        matches.push(matrix[i][column]);
                    }
                }
            }
        }
    }
    return matches;
}
