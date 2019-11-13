\null\clearpage

Useful Algorithms
========================================

Path Finding
-------------

Path Finding is that part of AI algorithms that takes care of getting from point A to point B, using the shortest way possible and avoiding obstacles.

### Representing our world

Before undertaking the concept of path finding algorithms, we need to decide in which way we should represent our world to the AI we want to code. There are different ways to do it and here we will discuss some of the most used ones.

#### 2D Grids

The simplest way to represent a world to an AI is probably using 2D grids: we can represent the world using a 2-dimensional matrix, where every cell is a free space, an obstacle, a start or goal cell.

This representation works well with top-down tile-based 2D games (with tile-based or free movement).

\placeholder

#### Path nodes

A more flexible way to represent our world is using "Path nodes", where each "path node" is represented by a node in a graph.

Graphs can be represented in code in two common ways (there are surely other ways to do so): using *adjacency lists* or using *adjacency matrices*.

This type of graph-based abstraction is the most used when teaching path finding algorithms like `A*` or `Dijkstra`.

\placeholder

#### Navigation meshes

Navigation meshes are used to solve a problem that can arise when we try to represent our world using path nodes: we can't represent "safe areas" (where the AI-driven entity can cross) without using possibly thousands of path nodes.

Navigation meshes are constituted by a collection of convex polygons (the meshes) that define the "safe areas", each mesh has no obstructions inside of itself, so the AI-driven entity can safely cross the entire mesh freely.

This abstraction allows to use `A*` and `Dijkstra` algorithms, but instead of trying to navigate a graph, you look for a path between meshes (which are saved in a graph structure).

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

Since all "possibly shortest" paths between two points have the same Manhattan distance, this guarantees us that the algorithm will never overestimate (all the other paths will be longer, so the Manhattan distance will underestimate the cost), which is required for this heuristic to be considered "admissible".

\code{algorithms/manhattan_distance}{Example code calculating the Manhattan distance on a 2D grid}

This works well with 2D grid-based worlds.

#### Euclidean Distance heuristic

Euclidean Distance works well when diagonal movement in a 2D grid is allowed, Euclidean distance is calculated with the standard distance formula:

$$ h(x) = \sqrt{(start.x - end.x) ^2 + (start.y - end.y)^2}$$

![Example of Euclidean Distance](./images/algorithms/euclidean_distance.png){width=30%}

\code{algorithms/euclidean_distance}{Example code calculating the Euclidean distance on a 2D grid}

### Algorithms

Before getting to the algorithms, we need to consider two supporting data structures that we will use:

- **Open Set:** a sorted data structure that contains the nodes that currently need to be considered. It should be an heap or any kind of structure that can be quickly be sorted as we will have to often refer to the node/cell/mesh with the lowest heuristic.
- **Closed Set:** a data structure that will contain all the nodes/meshes/cells that have already been considered by the algorithm. This structure should be easily searchable (like a binary search tree), since we will often need to see if a certain node/cell/mesh is part of this set.

#### The Greedy Best First Algorithm

This is a *greedy algorithm*~[g]~ that searches the "local best" (what is best in a certain moment, without planning future decisions) that makes use of heuristics.

For each of the neighbouring cells/meshes/nodes that have not been explored yet, the algorithm will take the one that has the lowest heuristic cost. Since this algorithm doesn't make any planning, this can lead to results that are not optimal, usually translating in entities hugging walls to reach their goal, as well as taking longer paths.

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
