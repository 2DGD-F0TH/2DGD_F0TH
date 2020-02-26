\null\clearpage

Other Useful Algorithms
========================================

\epigraph{Fancy algorithms are slow when n is small, and n is usually small.}{\textit{Rob Pike}}

World Generation
----------------

### Maze Generation

Maze generation is the base of a great majority of dungeon generation systems, you can create a maze, carve out a few rooms, put an entrance and an exit and you have a nice quick dungeon!

There are many ideas that can be used to generate a maze, some are based on a prepared map that gets refined into a maze, some other are based on walls instead of tiles, here we will see some of the many algorithms that exist.

#### Randomized Depth-First Search (Recursive Backtracker)

The Depth-First Search (DFS) algorithm is known in the world of tree and graph structure as a traversal algorithm. We can use a randomized DFS algorithm as a simple maze-generation algorithm.

The Randomized DFS Algorithm is usually implemented using the backtracking technique and a recursive routine: such algorithm is called "Recursive Backtracker".

The idea behind the algorithm is, starting from a defined "cell", to explore the grid randomly by choosing an available direction, digging a path.

![How the recursive backtracker algorithm works (1)](./images/algorithms/RDFS_1.png){width=30%}

When the algorithm detects that there is no available direction that means that the "head" of our digger is hitting against already explored cells or the map borders.

![How the recursive backtracker algorithm works (2)](./images/algorithms/RDFS_2.png){width=30%}

In such case, we "backtrack" until we find a cell with at least one available direction and continue our exploration.

![How the recursive backtracker algorithm works (3)](./images/algorithms/RDFS_3.png){width=30%}

This "digging and backtracking" keeps going until there are no other cells that have not been visited.

![How the recursive backtracker algorithm works (4)](./images/algorithms/RDFS_4.png){width=30%}

In some versions of the algorithm we need to also keep track of cells that will be used as "walls", so the actual implementation varies.

<!--TODO: System-Stack Recursive backtracker version -->

\placeholder

This algorithm can involve a big deal of recursion, which can lead to a *stack overflow*~[g]~ in your program, stopping the algorithm from working and your game in its entirety. It is possible to work around this issue by using an explicit stack, instead of using the call stack.

<!--TODO: Explicit stack recursive backtracker version -->
\placeholder

This algorithm, being taken from a Depth-First search algorithm, is biased towards creating very long corridors.

#### Randomized Kruskal's Algorithm

This algorithm is based on a randomized version of the minimum-spanning tree algorithm known as Kruskal's algorithm.

The algorithm needs the following data structures to work:

- One structure that contains all the "walls" of our maze, this can be a list
- One structure that allows for easy joining of disjoint sets, this will contain the cells

Initially all the cells are separated by walls, and each cell is its own set.

![How the Randomized Kruskal's Algorithm Works (1/6)](./images/algorithms/Kruskal1.png){width=30%}

Now we select a random wall from our list, if the cells separated by such wall are part of different sets, we delete the wall and join the cells into a single set.

![How the Randomized Kruskal's Algorithm Works (2/6)](./images/algorithms/Kruskal2.png){width=30%}

The "different sets" check allows us to avoid having loops in our maze (and also deleting all the walls, in some cases). Next we select another wall, check if the cells divided by the wall are from different sets and join them.

![How the Randomized Kruskal's Algorithm Works (3/6)](./images/algorithms/Kruskal3.png){width=30%}

This doesn't look much like a maze yet, but by uniting the cells we can start seeing some short paths forming in our maze.

![How the Randomized Kruskal's Algorithm Works (4/6)](./images/algorithms/Kruskal4.png){width=30%}

The black cells are starting to develop a path, as stated earlier. As the sets get bigger, there will be less walls we can "break down" to join our sets.

![How the Randomized Kruskal's Algorithm Works (5/6)](./images/algorithms/Kruskal5.png){width=30%}

When there is only one set left, our maze is complete.

![How the Randomized Kruskal's Algorithm Works (6/6)](./images/algorithms/Kruskal6.png){width=30%}

This algorithm, being based on a minimum-spanning tree algorithm, this algorithm is biased towards creating a large number of short dead ends.

Now let's see an example implementation of the Randomized Kruskal's Algorithm:

<!-- TODO: Code -->

\placeholder

### Recursive Division Algorithm

This algorithm is a bit similar to the recursive backtracker, but instead of focusing on passages, this algorithm focuses on walls: the idea is recursively dividing the space available with a horizontal or vertical wall which has a "hole" placed randomly.

This algorithm can give better results when the choice between "vertical" and "horizontal" walls is biased by the sizes of the sub-areas given by the last division.

Let's see how the algorithm works.

Starting from an empty maze, with no walls, we decide the direction (horizontal or vertical) of our first wall and add it, in a random position, making sure that there's an opening in such wall.

![How the Recursive Division Algorithm Works (1/6)](./images/algorithms/recursive_division_1.png){width=30%}

We select one of the two sub-areas we find, recursively and we add another wall in a random position and with a random direction.

![How the Recursive Division Algorithm Works (2/6)](./images/algorithms/recursive_division_2.png){width=30%}

We select one of the two sub-sub-area, and add another wall, with a random position and direction.

![How the Recursive Division Algorithm Works (3/6)](./images/algorithms/recursive_division_3.png){width=30%}

We keep on diving each sub-area recursively, adding walls, until the sub-area had one of its 2 dimensions (horizontal or vertial) equal to 1 cell.

![How the Recursive Division Algorithm Works (4/6)](./images/algorithms/recursive_division_4.png){width=30%}

When that happens, we backtrack to one of the previous sub-sections and continue.

![How the Recursive Division Algorithm Works (5/6)](./images/algorithms/recursive_division_5.png){width=30%}

This keeps going until the maze is complete.

![How the Recursive Division Algorithm Works (6/6)](./images/algorithms/recursive_division_6.png){width=30%}

Although it's one of the most efficient algorithms out there (considering that it can easily be converted to a multi-threaded version), given its nature, this algorithm is naturally biased towards building long walls, which give the maze a very "rectangle-like" feeling.

This bias is more noticeable with bigger mazes, like the following one.

![The bias of Recursive Division Algorithm](./images/algorithms/recursive_division_maze.png){width=60%}

Let's see an example implementation of this algorithm:

<!-- TODO: Code -->

\placeholder

### Binary Tree Algorithm

This is another very efficient "passage carver" algorithm: for each cell we carve a passage that either leads upwards or leftwards (but never both).

<!-- TODO: Detailed explanation -->

\placeholder

Given its deep roots into the computer science "Binary Tree" structure (where the root is the upper-left corner), this algorithm shows only half of the cell types available in mazes: there are no crossroads and all dead ends will either have a passage upwards or leftwards (but again, never both at the same time).

Let's see an example implementation of the "binary tree algorithm":

<!-- TODO: Code -->

\placeholder

### Eller's Algorithm

Eller's algorithm is the most memory-efficient maze-generation algorithm known so far: you generate the maze row-by-row, without needing to memorize the whole maze in memory while creating it.

<!-- TODO: Detailed explanation -->

\placeholder

Let's see a possible implementation of this strange, but interesting algorithm:

<!-- TODO: Code -->

\placeholder

Noise Generation
-----------------

"Noise" can be a very important part of game development: we can create textures with it, or even use it to generate worlds. In this section we will take a look at how to create "noise" efficiently and with the desired result: from completely randomized to more "natural looking" noise we can use to create maps.

### Randomized Noise (Static)

The simplest kind of noise we can generate is also known as "static", for each unit of our elaboration (it can be a pixel, for instance), we generate a random number between two bounds.

We can create some "TV-like" static with a few lines of code, like the following:

<!-- TODO: Code -->

\placeholder

### Perlin Noise

\placeholder

<!-- TODO: Noise generation algorithm, runs in O(2^n) for n dimensions -->
