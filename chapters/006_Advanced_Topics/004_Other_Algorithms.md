{{pagebreak}}

Other Useful Algorithms
========================================

:::::: {.epigraph author="Rob Pike"}
Fancy algorithms are slow when n is small, and n is usually small.
::::::

World Generation
----------------

### Midpoint Displacement Algorithm

As the name implies, this algorithm entails recursively taking the midpoint between two extremes and "displace" it.

Let's see how it works first. We have a completely flat terrain:

![How the Midpoint Displacement Algorithm Works (1/4)](images/algorithms/MDA_1.svg){width=50%}

Then we take the midpoint between the two extremes and move it (up or down) by a random amount (between two sensible extremes):

![How the Midpoint Displacement Algorithm Works (2/4)](images/algorithms/MDA_2.svg){width=50%}

In this case we moved it up by a certain measure, now we take the two midpoints on the left and right sides of the previous midpoint, and displace them too:

![How the Midpoint Displacement Algorithm Works (3/4)](images/algorithms/MDA_3.svg){width=50%}

Then we take the midpoints between the segments we created, and displace them again:

![How the Midpoint Displacement Algorithm Works (4/4)](images/algorithms/MDA_4.svg){width=50%}

Each displacement is usually done by a lower amount of the previous ones, so that the first displacements give a "general shape" of the terrain, while the ones further down the line are going to give "detail" to our terrain.

The algorithm terminates when we reached a pre-defined number of subdivisions, sometimes called "octaves". In the previous example, we have 4 octaves.

Let's take a look at a possible implementation of the midpoint displacement algorithm:

```{src='algorithms/midpoint_displacement' caption='Example implementation of the midpoint displacement algorithm'}
```

::: note :::
This algorithm can be extended to create 2D height-maps (which can be used in turn to create 3D ground) and noise textures quite easily, but it also presents some artifacts that can be noticeable. The diamond-square algorithm solves this issue.
::::::::::::

### Diamond-Square Algorithm

The diamond-square algorithm is an evolution in 2D of the midpoint displacement algorithm (so far, we just changed one value, in one dimension).

The algorithm iteratively repeats two steps:

- a **diamond step** where you find all the squares, take the midpoint and set it as the average of the four corners, plus a random displacement;
- a **square step** where you find all the diamonds, take the midpoint and set it as the average of the four corners, plus a random displacement.

Let's see how it works.

First of all, we bootstrap our square with arbitrary values at the four corners, this will be our starting point.

![How the diamond-square algorithm works (1/5)](images/algorithms/diamond_square_1.png){width=50%}

Now we perform the first "diamond step": we have only one big square, with 4 corners, we identify its center and set its value to the average of the corners and apply a random displacement.

![How the diamond-square algorithm works (2/5)](images/algorithms/diamond_square_2.png){width=50%}

Now we can perform our first "square step": we have 4 diamonds, we identify their centres (in red) and set them to the average of their neighbours, plus a random displacement.

![How the diamond-square algorithm works (3/5)](images/algorithms/diamond_square_3.png){width=50%}

Now we can iterate with another "diamond step" on the four new smaller squares we have, get their centers and again average and displace.

![How the diamond-square algorithm works (4/5)](images/algorithms/diamond_square_4.png){width=50%}

As the last step, for this 5x5 grid, we perform another "square step", finding 12 smaller diamonds, and again getting the average of the corners and apply a displacement.

![How the diamond-square algorithm works (5/5)](images/algorithms/diamond_square_5.png){width=50%}

{{placeholder}}

<!-- TODO: Evolution of the Midpoint displacement, usable for 2D height maps -->

### Maze Generation

Maze generation is the base of a great majority of dungeon generation systems, you can create a maze, carve out a few rooms, put an entrance and an exit and you have a nice quick dungeon!

There are many ideas that can be used to generate a maze, some are based on a prepared map that gets refined into a maze, some other are based on walls instead of tiles, here we will see some of the many algorithms that exist.

#### Randomized Depth-First Search (Recursive Backtracker)

The Depth-First Search (DFS) algorithm is known in the world of tree and graph structure as a traversal algorithm. We can use a randomized DFS algorithm as a simple maze-generation algorithm.

The Randomized DFS Algorithm is usually implemented using the backtracking technique and a recursive routine: such algorithm is called "Recursive Backtracker".

The idea behind the algorithm is, starting from a defined "cell", to explore the grid randomly by choosing an available direction, digging a path.

![How the recursive backtracker algorithm works (1)](./images/algorithms/RDFS_1.svg){width=30%}

When the algorithm detects that there is no available direction that means that the "head" of our digger is hitting against already explored cells or the map borders.

![How the recursive backtracker algorithm works (2)](./images/algorithms/RDFS_2.svg){width=30%}

In such case, we "backtrack" until we find a cell with at least one available direction and continue our exploration.

![How the recursive backtracker algorithm works (3)](./images/algorithms/RDFS_3.svg){width=30%}

This "digging and backtracking" keeps going until there are no other cells that have not been visited.

![How the recursive backtracker algorithm works (4)](./images/algorithms/RDFS_4.svg){width=30%}

In some versions of the algorithm we need to also keep track of cells that will be used as "walls", so the actual implementation varies.

{{placeholder}}

<!-- TODO: System-Stack Recursive backtracker version code -->

This algorithm can involve a big deal of recursion, which can lead to a *stack overflow*~[g]~ in your program, stopping the algorithm from working and your game in its entirety. It is possible to work around this issue by using an explicit stack, instead of using the call stack.

{{placeholder}}

<!-- TODO: Explicit stack recursive backtracker version -->

This algorithm, being taken from a Depth-First search algorithm, is biased towards creating very long corridors.

#### Randomized Kruskal's Algorithm

This algorithm is based on a randomized version of the minimum-spanning tree algorithm known as Kruskal's algorithm.

The algorithm needs the following data structures to work:

- One structure that contains all the "walls" of our maze, this can be a list
- One structure that allows for easy joining of disjoint sets, this will contain the cells

Initially all the cells are separated by walls, and each cell is its own set.

![How the Randomized Kruskal's Algorithm Works (1/6)](./images/algorithms/Kruskal1.svg){width=30%}

Now we select a random wall from our list, if the cells separated by such wall are part of different sets, we delete the wall and join the cells into a single set.

![How the Randomized Kruskal's Algorithm Works (2/6)](./images/algorithms/Kruskal2.svg){width=30%}

The "different sets" check allows us to avoid having loops in our maze (and also deleting all the walls, in some cases). Next we select another wall, check if the cells divided by the wall are from different sets and join them.

![How the Randomized Kruskal's Algorithm Works (3/6)](./images/algorithms/Kruskal3.svg){width=30%}

This doesn't look much like a maze yet, but by uniting the cells we can start seeing some short paths forming in our maze.

![How the Randomized Kruskal's Algorithm Works (4/6)](./images/algorithms/Kruskal4.svg){width=30%}

The black cells are starting to develop a path, as stated earlier. As the sets get bigger, there will be less walls we can "break down" to join our sets.

![How the Randomized Kruskal's Algorithm Works (5/6)](./images/algorithms/Kruskal5.svg){width=30%}

When there is only one set left, our maze is complete.

![How the Randomized Kruskal's Algorithm Works (6/6)](./images/algorithms/Kruskal6.svg){width=30%}

Being based on a minimum-spanning tree algorithm, this algorithm is biased towards creating a large number of short dead ends.

Now let's see an example implementation of the Randomized Kruskal's Algorithm:

{{placeholder}}

<!-- TODO: Code for randomized kruskal's method -->

#### Recursive Division Algorithm

This algorithm is a bit similar to the recursive backtracker, but instead of focusing on passages, this algorithm focuses on walls: the idea is recursively dividing the space available with a horizontal or vertical wall which has a "hole" placed randomly.

This algorithm can give better results when the choice between "vertical" and "horizontal" walls is biased by the sizes of the sub-areas given by the last division.

Let's see how the algorithm works.

Starting from an empty maze, with no walls, we decide the direction (horizontal or vertical) of our first wall and add it, in a random position, making sure that there's an opening in such wall.

![How the Recursive Division Algorithm Works (1/6)](./images/algorithms/recursive_division_1.svg){width=30%}

We select one of the two sub-areas we find, recursively and we add another wall in a random position and with a random direction.

![How the Recursive Division Algorithm Works (2/6)](./images/algorithms/recursive_division_2.svg){width=30%}

We select one of the two sub-sub-area, and add another wall, with a random position and direction.

![How the Recursive Division Algorithm Works (3/6)](./images/algorithms/recursive_division_3.svg){width=30%}

We keep on diving each sub-area recursively, adding walls, until the sub-area had one of its 2 dimensions (horizontal or vertial) equal to 1 cell.

![How the Recursive Division Algorithm Works (4/6)](./images/algorithms/recursive_division_4.svg){width=30%}

When that happens, we backtrack to one of the previous sub-sections and continue.

![How the Recursive Division Algorithm Works (5/6)](./images/algorithms/recursive_division_5.svg){width=30%}

This keeps going until the maze is complete.

![How the Recursive Division Algorithm Works (6/6)](./images/algorithms/recursive_division_6.png){width=30%}

Although it's one of the most efficient algorithms out there (considering that it can easily be converted to a multi-threaded version), given its nature, this algorithm is naturally biased towards building long walls, which give the maze a very "rectangle-like" feeling.

This bias is more noticeable with bigger mazes, like the following one.

![The bias of Recursive Division Algorithm](./images/algorithms/recursive_division_maze.png){width=60%}

Let's see an example implementation of this algorithm:

{{placeholder}}

<!-- TODO: Code for the recursive division algorithm -->

#### Binary Tree Algorithm

This is another very efficient "passage carver" algorithm: for each cell we carve a passage that either leads upwards or leftwards (but never both).

Let's see how the algorithm works: we start from the bottom-right cell of the maze (the last one).

![How the Binary Tree Maze generation works (1/6)](./images/algorithms/bintree_1.svg){width=30%}

Now we decide, randomly, to carve a passage either upwards or leftwards (we will not carve a passage that "creates a hole in a wall"). In this case we go leftwards.

![How the Binary Tree Maze generation works (2/6)](./images/algorithms/bintree_2.svg){width=30%}

Now let's go to the second-to-last cell...

![How the Binary Tree Maze generation works (3/6)](./images/algorithms/bintree_3.svg){width=30%}

And again, decide randomly to carve a passage either upwards or leftwards, this time we chose upwards.

![How the Binary Tree Maze generation works (4/6)](./images/algorithms/bintree_4.svg){width=30%}

Again we go to the "previous" cell and continue with our process, until we hit the left wall (which will force us to carve a passage upwards) or the top wall (which will force us to go left); when we hit both the top and the left walls, we stop.

![How the Binary Tree Maze generation works (5/6)](./images/algorithms/bintree_5.svg){width=30%}

Here is the result of the algorithm:

![How the Binary Tree Maze generation works (6/6)](./images/algorithms/bintree_6.svg){width=30%}

Given its deep roots into the computer science "Binary Tree" structure (where the root is the upper-left corner), this algorithm shows only half of the cell types available in mazes: there are no crossroads and all dead ends will either have a passage upwards or leftwards (but again, never both at the same time).

Let's see an example implementation of the "binary tree algorithm":

{{placeholder}}

<!-- TODO: Code for the binary tree algorithm -->

#### Eller's Algorithm

Eller's algorithm is the most memory-efficient maze-generation algorithm known so far: you generate the maze row-by-row, without needing to memorize the whole maze in memory while creating it.

To start, we decide the width and height of the maze, and create a single row, large as the width we want. Then we set each cell to be its own "set".

![How Eller's Maze Generation Algorithm Works (1/7)](./images/algorithms/eller_1.svg){width=30%}

Now we scroll through the cells and randomly join adjacent cells that are part of two different "sets".

![How Eller's Maze Generation Algorithm Works (2/7)](./images/algorithms/eller_2.svg){width=30%}

After joining we create some "holes" in the bottom wall, making sure that each "set" has at least one hole to get to the next row.

![How Eller's Maze Generation Algorithm Works (3/7)](./images/algorithms/eller_3.svg){width=30%}

After that we start creating the next row, connecting the cells that have a "hole" with the previous row and assigning them the same set. In the picture the gray cells didn't get a set assigned yet.

![How Eller's Maze Generation Algorithm Works (4/7)](./images/algorithms/eller_4.svg){width=30%}

After that we assign a new set to the remaining cells.

![How Eller's Maze Generation Algorithm Works (5/7)](./images/algorithms/eller_5.svg){width=30%}

At this point we just need to iterate, ignoring the previous row: we join adjacent cells that are not part of the same "set" (we grayed out the previous row).

![How Eller's Maze Generation Algorithm Works (6/7)](./images/algorithms/eller_6.svg){width=30%}

Then we create "holes" for each set and prepare the next row. In case we want the maze to be wholly interconnected then if the row is the last row, we can just join all the cells.

![How Eller's Maze Generation Algorithm Works (7/7)](./images/algorithms/eller_7.svg){width=30%}

Obviously we can repeat the iteration as many times as we want, and we get a maze as big as we want. This algorithm has no obvious biases and is good for very efficient dungeon generation, if you add rooms, for instance.

Let's see a possible implementation of this strange, but interesting algorithm:

{{placeholder}}

<!-- TODO: Code for the Eller's Algorithm -->

Dungeon Generation
------------------

{{placeholder}}

<!-- TODO: Talk about some algorithms that can be used to generate dungeon rooms in the style of Binding of Isaac: first you generate the "big map", then each room is chosen among a pool of pre-made rooms that can be re-rextured -->

Noise Generation
-----------------

"Noise" can be a very important part of game development: we can create textures with it, or even use it to generate worlds. In this section we will take a look at how to create "noise" efficiently and with the desired result: from completely randomized to more "natural looking" noise we can use to create maps.

### Randomized Noise (Static)

The simplest kind of noise we can generate is also known as "static", for each unit of our elaboration (it can be a pixel, for instance), we generate a random number between two bounds.

Here is an example of random noise:

![Example of Random Noise](./images/algorithms/random_noise.png){width=50%}

We can create some "TV-like" static with a few lines of code, like the following:

```{src='algorithms/randomized_noise' caption='Example implementation of randomized noise'}
```

### Perlin Noise

{{placeholder}}

<!-- TODO: Noise generation algorithm, runs in O(2^n) for n dimensions -->

Animation
---------

### Skeleton animation

{{placeholder}}

<!-- TODO: Talk about skeleton animation, keyframes, joints and how it can be used to make "big bosses" that use many parts without having to use giant (and heavy) sprites -->
