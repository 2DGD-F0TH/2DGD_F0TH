function findHorizontalMatches(){
    let matchLength = 0;
    const minMatchLength = 3;
    matches = [];
    for (const row in matrix){
        let lastMatchingTile = null;
        for (const column in row){
            let currentTile = matrix[row][column].tile;
            if (currentTile == lastMatchingTile){
                matchLength = matchLength + 1;
            }else{
                if (matchLength >= minMatchLength){
                    // We need to memorize all the tiles involved in the match
                    for (let i = column - matchLength; i <= column; i++){
                        matches.push(matrix[row][i]);
                    }
                }else{
                    // No matches, reset the counter and set the current tile as last matching
                    matchLength = 1;
                    lastMatchingTile = currentTile;
                }
            }
            // We need to account for the right-hand border corner case
            if (column == matrix[row].length){
                if (matchLength >= minMatchLength){
                    // We need to memorize all the tiles involved in the match
                    for (let i = column - matchLength; i <= column; i++){
                        matches.push(matrix[row][i]);
                    }
                }
            }
        }
    }
    return matches;
}
