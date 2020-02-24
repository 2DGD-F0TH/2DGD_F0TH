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

\placeholder

Noise Generation
-----------------

### Perlin Noise

\placeholder

<!-- TODO: Noise generation algorithm, runs in O(2^n) for n dimensions -->
