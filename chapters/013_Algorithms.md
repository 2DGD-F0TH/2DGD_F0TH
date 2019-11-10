\null\clearpage

Useful Algorithms
========================================

Path Finding
-------------

### Representing our world

\placeholder

#### Path nodes

\placeholder

#### Navigation meshes

\placeholder

### Heuristics

In path finding there can be "heuristics" that are accounted for when you have to take a decision: in path finding an heuristic $h(x)$ is an estimated cost to travel from the current node to the goal node.

An heuristic is admissible if it *never overestimates* the cost: if it did, it wouldn't guarantee that the algorithm would find the best path to the goal node.

In this book we will present the most common heuristics used in game development.

#### Manhattan Distance heuristic

\placeholder

#### Euclidean Distance heuristic

\placeholder

### Algorithms

#### The Best Greedy First Algorithm

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
