\null\clearpage

Useful Algorithms
========================================

Path Finding
-------------

### Representing our world

<!-- TODO: Introduction on how to represent the world for a pathfinding AI -->

\placeholder

#### 2D Grids

<!-- TODO: Talk about representing the world with 2D grids (Matrix) -->

\placeholder

#### Path nodes

<!-- TODO: Talk about how to represent the world via nodes of a graph, adjacency lists and adjacency matrices -->

\placeholder

#### Navigation meshes

<!-- TODO: Talk about navigation meshes -->

\placeholder

### Heuristics

In path finding there can be "heuristics" that are accounted for when you have to take a decision: in path finding an heuristic $h(x)$ is an estimated cost to travel from the current node to the goal node.

An heuristic is admissible if it *never overestimates* such cost: if it did, it wouldn't guarantee that the algorithm would find the best path to the goal node.

In this book we will present the most common heuristics used in game development.

#### Manhattan Distance heuristic

The Manhattan Distance heuristic doesn't allow diagonal movement (allowing it would allow the heuristic to overestimate the cost), and for a 2D grid is formulated as follows:

$$ h(x) = | start.x - goal.x | + | start.y - goal.y | $$

Graphically:

![Example of Manhattan distance](./images/algorithms/manhattan_distance.png){width=60%}

On the left we can see how we calculate the Manhattan distance, on the right you can notice how all the "possibly shortest" alternative paths have the same Manhattan distance.

Since all "possibly shortest" paths between two points have the same Manhattan distance, this guarantees us that the algorithm will never overestimate, which is required for this heuristic to be considered "admissible".

\code{algorithms/manhattan_distance}{Example code calculating the Manhattan distance on a 2D grid}

This works well with 2D grid-based worlds.

#### Euclidean Distance heuristic

Euclidean Distance works well when diagonal movement in a 2D grid is allowed, Euclidean distance is calculated with the standard distance formula:

$$ h(x) = \sqrt{(start.x - end.x) ^2 + (start.y - end.y)^2}$$

![Example of Euclidean Distance](./images/algorithms/euclidean_distance.png){width=30%}

\code{algorithms/euclidean_distance}{Example code calculating the Euclidean distance on a 2D grid}

### Algorithms

#### The Greedy Best First Algorithm

\placeholder

<!-- TODO: Uses an heuristic to understand which node is the "best" locally, but doesn't necessarily make the best decision globally -->

#### The Dijkstra Algorithm

\placeholder

<!-- TODO: Explain the dijkstra algorithm on nodes of a graph, can detect the closest goal among many -->


#### The A* Algorithm

\placeholder

<!-- TODO: Variation of Dijkstra that uses heuristics for faster processing, does the best globally but doesn't support more than one goal -->

World Generation
----------------

### Recursive Backtracker Algorithm

\placeholder

<!-- TODO: Introduce the user to maze generation with the recursive backtracker algorithm -->

Noise Generation
-----------------

### Perlin Noise

\placeholder

<!-- TODO: Noise generation algorithm, runs in O(2^n) for n dimensions -->
