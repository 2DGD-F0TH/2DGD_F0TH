\null\clearpage
Genre-Specific Tips, Tricks and Algorithms
==========================================

2D Platformers
---------------

### Simulating Gravity

\placeholder

<!-- TODO: Just add a constant acceleration down -->

### Parallax Scrolling

\placeholder

<!-- TODO: talk about image "loop points", how big the image should be (at least 2x the screen size in the scrolling direction) and how to loop and make a seemingly infinite background-->

### Ladders

\placeholder

<!-- TODO: How to allow the player to use ladders -->

### Walking on slanted ground

\placeholder

<!-- TODO: How to walk on slanted ground -->

### Stairs

\placeholder

<!-- TODO: How to walk on stairs -->

Top-view RPG-Like Games
-----------------------

### Managing height

\placeholder

<!-- TODO: How to manage the different "heights" in the game -->

Rhythm Games
------------

### Synchronizing with the Music

\placeholder

<!-- TODO: Basic Beat detection or how to somehow sync music and gameplay -->

Match-x Games
--------------

### Managing and drawing the grid

When it comes to a match-x game, a good data structure for the play field is a matrix.

In most programming languages, a matrix is saved as an "array of arrays", where you have each element of an array representing a row, and each element of a row is a tile.

![Example of a matrix, saved as "array of arrays"](./images/specific_genre/array_arrays.pdf){width=60%}

This is a really nice way to interpret a grid, but at the same time it can open the door to some pitfalls if you're not careful.

Many programming languages allow for direct access to an element inside an "array of arrays" by using multiple access operators (usually the `[]` operator) in a row.

Usually each element you access with the first `[]` operator represents the rows, while the second time you use `[]` you will access the columns, this will make it so you need to access an element directly as follows: `matrix[y][x]` where "y" is the row number and "x" is the column number, which can prove counter-intuitive.

In the previous example, if we want to access the highlighted item, at the third row (indexed at 2), and in the fifth column (indexed at 4). We have to use `matrix[2][4]`, which is the opposite of what many people are used to when they think in `(x,y)` coordinates.

Try to keep visual representation and data structures separated in your mind, to avoid confusion.

### Finding and removing Matches

If you're doing a simple match-x game where you can only match tiles horizontally or vertically, the algorithm to find matches is quite simple, both conceptually and computationally.

The main idea is dividing the "horizontal matches" from the "vertical" ones. This will allow to simplify the algorithm and avoid some pitfalls (unless you want to give bonuses for "T-shaped" and "L shaped" matches).

For horizontal matches the idea is running through each row, keeping some variables representing the length of the match, as well as the color of the current "ongoing" match. As soon as we find a different color, if the length of the "ongoing" match is higher than "x" (usually 3), we save the references to the tiles involved for later removal.

Similarly we can do the same algorithm for vertical matches, by running through each column and saving the matches.

Here is a pseudo-code example:

~~~~
function findHorizontalMatches():
    matchLength = 0
    minMatchLength = 3
    for each row in matrix:
        lastMatchingTile = null
        for each column in row:
            currentTile = matrix[row][column].tile
            if currentTile == lastMatchingTile:
                matchLength = matchLength + 1
            else:
                if matchLength >= minMatchLength:
                    // We need to memorize all the tiles involved in the match
                    for each tile from matrix[row][column-matchLength] to matrix[row][column]:
                        memorize(tile)
                else:
                    // No matches, reset the counter and set the current tile as last matching
                    matchLength = 1
                    lastMatchingTile = currentTile
            // We need to account for the right-hand border corner case
            if column == size(matrix[row]):
                if matchLength >= minMatchLength:
                    // We need to memorize all the tiles involved in the match
                    for each tile from matrix[row][column-matchLength] to matrix[row][column]:
                        memorize(tile)
~~~~

Let's talk a second about the last rows in the algorithm: they are specifically tailored to address a corner case that happens when there is a match that ends on the right border of the screen.

If such code was not there, the match number would grow by one, then the for loop would reset everything and we'd lose such match.

Similarly, we can make an algorithm that allows for vertical matches to be memorized for later removal:

~~~~
function findVerticalMatches():
    matchLength = 0
    minMatchLength = 3
    for each column in matrix:
        lastMatchingTile = null
        for each row in column:
            currentTile = matrix[row][column].tile
            if currentTile == lastMatchingTile:
                matchLength = matchLength + 1
            else:
                if matchLength >= minMatchLength:
                    // We need to memorize all the tiles involved in the match
                    for each tile from matrix[row-matchLength][column] to matrix[row][column]:
                        memorize(tile)
                else:
                    // No matches, reset the counter and set the current tile as last matching
                    matchLength = 1
                    lastMatchingTile = currentTile
            // We need to account for the bottom border corner case
            if row == size(matrix[column]):
                if matchLength >= minMatchLength:
                    // We need to memorize all the tiles involved in the match
                    for each tile from matrix[row-matchLength][column] to matrix[row][column]:
                        memorize(tile)
~~~~

Both algorithms run in $O(n)$, where "n" is the number of tiles on the screen.

Now we can proceed to remove every tile that has been memorized as "part of a match", the quickest way may be to set such tile to "null" (or an equivalent value for your programming language).

#### Why don't we delete the matches immediately?

We could, but that would open the door to a pitfall that could be tough to manage: in case of a "T" match, we would find that the "horizontal matches" algorithm deletes part of said match, and the "vertical matches" algorithm wouldn't be able to complete the "T match", because the necessary tiles are deleted.

Let's see the image to understand better:

![What happens when deleting a match immediately](./images/specific_genre/immediate_deletion.pdf){width=60%}

As visible from the first image, there is a T-shaped match involving cells 0,1,2 of row 0, cell 1 of row 1 and cell 1 of row 2.

If we deleted the horizontal match immediately, we would lose the possibility of completing the vertical match (highlighted in the second image).

Instead we memorize everything first, and then delete all the matches at once, without the risk of losing anything.

### Replacing the removed tiles and applying gravity

\placeholder

<!-- TODO: Make the tiles fall by instantly setting the tile IDs in the matrix but tween the graphics -->
