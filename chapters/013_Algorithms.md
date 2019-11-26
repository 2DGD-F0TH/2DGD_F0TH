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

\code{algorithms/2d_grid}{Possible representation of a 2D grid}

Even though this is probably the most straightforward way to represent a world in many cases, most of the algorithms used work on a graph structure instead of a 2D grid. It shouldn't be too hard to adapt the algorithms presented here to work with this structure.

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

We will reference this image when we check what path the algorithm will take:

![Pathfinding Algorithms Reference Image](./images/algorithms/pathfinding_reference.png){width=30%}

The shaded rows represent the indexes we'll refer to when operating the algorithm.

In each algorithm there will be the path taken by its implementation, so we invite you to execute the algorithm's instructions by hand, taking account of the heuristics pre-calculated and shown in the images below.

![Pathfinding Algorithms Heuristics Reference Image](./images/algorithms/pathfinding_heuristics.png){width=60%}

#### The Greedy "Best First" Algorithm

This is a *greedy algorithm*~[g]~ that searches the "local best" (what is best in a certain moment, without planning future decisions) that makes use of heuristics.

For each of the neighbouring cells/meshes/nodes that have not been explored yet, the algorithm will take the one that has the lowest heuristic cost. Since this algorithm doesn't make any planning, this can lead to results that are not optimal, usually translating in entities hugging walls to reach their goal, as well as taking longer paths.

In this algorithm we will use a "Node" structure composed as follows:

\code{algorithms/greedy_node_structure}{The node structure used in the greedy "Best First" algorithm}

Let's look at the algorithm itself:

\code{algorithms/greedy_best_first}{The greedy "Best First" algorithm}

An interesting idea to optimize this algorithm and avoid the final "stack reversal" would be to find the path starting from the end node, towards the starting node.

In the image below we can see the path taken by the algorithm, and how it is not the most optimal path.

![The path taken by the greedy "Best First" algorithm](./images/algorithms/greedy_best_first_path.png){width=30%}

#### The Dijkstra Algorithm

The idea behind the Dijkstra algorithm is having a "cost" component that expresses the cost that has to be incurred when traveling from the start node to the current node. This will allow our previous algorithm to evolve and take the shortest path possible.

To be able to keep track of such "path-cost" component, we will use a different "Node" structure from the one used in the greedy "best-first" algorithm:

\code{algorithms/dijkstra_node_structure}{The node structure used in the Dijkstra Algorithm}

The idea behind the whole algorithm is that "if we find a quicker way to get from the start node to the current node, we should take it".

Let's take a look at the algorithm:

\code{algorithms/dijkstra}{The Dijkstra Algorithm}

As with the greedy "best-first" algorithm we can optimize the "stack reversal" stage by starting from the end node.

Below we can see the path taken by the algorithm:

![The path taken by the Dijkstra Algorithm](./images/algorithms/dijkstra_path.png){width=30%}

#### The A* Algorithm

The A* Algorithm joins the "path-cost" idea with the heuristic to have a more efficient path-finding algorithm.

The algorithm is really similar to Dijkstra, but it orders the open set by a new formula $f$, that is calculated as follows:

$$ f(x) = g(x) + h(x)$$

Where $g(x)$ is our path cost and $h(x)$ is the heuristic we selected.

Let's see the code:

\code{algorithms/a_star}{The A* Algorithm}

The path taken by the A* Algorithm is exactly the same as the one taken by the Dijkstra Algorithm, but the heuristic helps the A* Algorithm in visiting less nodes than its counterpart.

##### Dijkstra Algorithm as a special case of the A* Algorithm

The Dijkstra Algorithm can be implemented with the same code as the A* Algorithm, just by keeping the heuristic cost $h(x) = 0$.

The absence of the heuristics (which depends on the goal node) leads the Dijkstra Algorithm to visit more nodes, but it can be useful in case there are many valid goal nodes and we don't know which one is the closest.

World Generation
----------------

### Maze Generation

Maze generation is the base of a great majority of dungeon generation systems, you can create a maze, carve out a few rooms, put an entrance and an exit and you have a nice quick dungeon!

There are many ideas that can be used to generate a maze, some are based on a prepared map that gets refined into a maze, some other are based on walls instead of tiles, here we will see some of the many algorithms that exist.

#### Depth-First Search

The Depth-First Search (DFS) algorithm is known in the world of tree and graph structure as a traversal algorithm. We can use a randomized DFS algorithm as a simple maze-generation algorithm.

<!--TODO: Recursive DFS version -->

\placeholder

This algorithm can involve a big deal of recursion, which can lead to a *stack overflow*~[g]~ in your program, stopping the algorithm from working and your game in its entirety. It is possible to work around this issue by using an explicit stack, instead of using the call stack.

<!--TODO: Explicit stack DFS version -->
\placeholder

This algorithm, being taken from a Depth-First search algorithm, is biased towards creating very long corridors.

#### Recursive Backtracker Algorithm

The Recursive Backtracker Algorithm is one of the many algorithms used to generate mazes, in the case of a 2D grid this algorithm involves recursive calls to a function that takes care of carving a passage between two tiles.

<!-- TODO: Normal recursive backtracker -->
\placeholder

The issue with this algorithm is that it requires the full maze to be kept in memory, which could be a problem for larger worlds. Also, since this algorithm is based on recursion, there is a real danger of *stack overflow*~[g]~ that can be worked around by using an explicit stack.

<!--TODO: Recursive Backtracker with explicit stack and while loops -->

\placeholder

<!-- TODO: Introduce the user to maze generation with the recursive backtracker algorithm -->

#### Randomized Kruskal's Algorithm

This algorithm is based on a randomized version of the minimum-spanning tree algorithm known as Kruskal's algorithm.

\placeholder

This algorithm, being based on a minimum-spanning tree algorithm, is biased towards creating a big number of short dead ends.

Noise Generation
-----------------

### Perlin Noise

\placeholder

<!-- TODO: Noise generation algorithm, runs in O(2^n) for n dimensions -->
