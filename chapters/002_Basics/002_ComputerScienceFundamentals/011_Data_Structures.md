Data Structures
---------------

### Graphs {#graphs}

A graph is a data structure that contains a set of vertices (or nodes) which may be connected by a set of edges (or links).

Graphs can be represented in code in two common ways (there are surely other ways to do so): using *adjacency lists* or using *adjacency matrices*.

To explain the two main ways to represent graphs, we will use the following reference image:

![Graphical representation of a simple graph](./images/computer_science/graph_reference.svg){width=30%}

##### Adjacency Lists

Adjacency lists are very simple: they just list the "neighbours" inside a list-like container, every time the graph gets changed, so will the adjacency lists. This method is really flexible and easy to implement. In fact it can be represented in a simple table:

| Node   | Adjacency List   |
| :----: | :--------------- |
| `A`    | `[B, D]`         |
| `B`    | `[A, C, D]`      |
| `C`    | `[B]`            |
| `D`    | `[A, B, E]`      |
| `E`    | `[D]`            |

: A simple adjacency list for our reference image

{{placeholder}}

<!-- TODO: Example implementation for adjacency lists? -->

##### Adjacency Matrices

Another method is to use matrices as a way to store relations between nodes. We have an $n \times n$ matrix (where $n$ is the number of nodes involved) filled with zeros; we put a $1$ for every connection that the nodes have (in many conventions, self-loops use the value $2$).

Here is an example:

$$
\begin{bmatrix}
0 & 1 & 0 & 1 & 0\\
1 & 0 & 1 & 1 & 0\\
0 & 1 & 0 & 0 & 0\\
1 & 1 & 0 & 0 & 1\\
0 & 0 & 0 & 1 & 0
\end{bmatrix}
$$

If we label the matrix, things are a little bit easier to read:

+-------+-------+-------+-------+-------+-------+
|       | **A** | **B** | **C** | **D** | **E** |
+-------+-------+-------+-------+-------+-------+
| **A** |   0   |   1   |   0   |   1   |   0   |
+-------+-------+-------+-------+-------+-------+
| **B** |   1   |   0   |   1   |   1   |   0   |
+-------+-------+-------+-------+-------+-------+
| **C** |   0   |   1   |   0   |   0   |   0   |
+-------+-------+-------+-------+-------+-------+
| **D** |   1   |   1   |   0   |   0   |   1   |
+-------+-------+-------+-------+-------+-------+
| **E** |   0   |   0   |   0   |   1   |   0   |
+-------+-------+-------+-------+-------+-------+

: How to read an adjacency matrix

:::: note ::::
For non-directed graphs (like the one in the reference image), adjacency matrices are mirrored on the main diagonal. This may be useful information if you really want to squeeze the last bit of space out of your implementation.
::::::::::::::

Using the table, we can see that we have a $1$ in "row A, column B", which means there is a link "A to B", since there is a $1$ in "row B, column A", it means that there is a link "B to A" too. This makes it easy to store single-direction relationships (for Directed Graphs) in a compact way.

{{placeholder}}

<!-- TODO: Example implementation for adjacency matrices? -->

### Trees

When you are a programmer, sooner or later you will have to deal with trees: they are a data structure that represents a hierarchy, using a set of nodes.

Trees can be defined as a "recursive data structure", made up of a node and a bunch of sub-trees connected to it.

![Example of a tree structure](./images/computer_science/dfs_example.svg){width=40%}

The fact that we can define trees recursively also means that they're a good candidate for all kinds of recursive algorithms, which can help simplifying the code quite a bit.

Trees are the base structure for a lot of other data structures, like heaps and binary search trees.

In this book we will focus mostly on binary trees: trees where each node has at most 2 children.

A possible implementation of a tree could be the following:

```{src='computer_science/tree' caption='A possible implementation of a tree class'}
```

<!-- TODO: Tree code implementation -->

#### Depth-first Search

The Depth-first search is a so-called "tree traversal algorithm", which means that it's essentially a way to explore a tree structure. In this case, the algorithm will try to reach the nodes farthest from the root first, before "backtracking" (that means before going "back towards the root").

As said earlier, we will focus on binary trees.

![Order in which the nodes are visited during DFS](./images/computer_science/tree_traversal_dfs.svg){width=40%}

Depth-first search can be useful in the following situations (as well as others):

- Sorting;
- Maze generation (see the [Randomized DFS Method](#rdfs) in the [Maze generation](#mazegen) section);
- Maze solving (which may be useful for [Path finding](#pathfinding));

The DFS algorithm hides some subtleties, though: the algorithm will "traverse" the tree in the same order, but different implementations will "visit the tree nodes" differently. We will take a look at how nodes are visited now.

In explaining the DFS algorithm, we will refer to the example tree we saw earlier, here it is again:

![Example tree that will be traversed by DFS](./images/computer_science/dfs_example.svg){width=40%}

##### Pre-order Traversal

The pre-order traversal visits the current node before visiting its children. That means that the algorithm performs the following operations, in order:

1. Visit the current node
2. Recursively traverse the current node's left subtree
3. Recursively traverse the current node's right subtree

If we traverse the example tree with pre-order traversal, and print the visited node, the output will be: `GDBACFEHI`

Here is how an example implementation of a pre-order traversal of a binary tree using DFS would look like:

```{src='computer_science/dfs_preorder' caption='Pre-order traversal of a tree using DFS'}
```

##### In-order Traversal

The in-order traversal visits the tree "from left to right", by prioritizing the traversal of the left subtrees before visiting the current node. That means that the algorithm performs the following operations, in order:

1. Recursively traverse the current node's left subtree
2. Visit the current node
3. Recursively traverse the current node's right subtree

If we traverse the example tree with in-order traversal, and print the visited node, the output will be: `ABCDEFGHI`

Notice how in this case, the output is ordered. This is because the example tree is a special kind of tree, called a "binary search tree". We will see more in the [dedicated paragraph](#bst).

Here is how an example implementation of a in-order traversal of a binary tree using DFS would look like:

```{src='computer_science/dfs_inorder' caption='In-order traversal of a tree using DFS'}
```

##### Post-order Traversal

The post-order traveral method prioritizes traversing both the children to visiting the current node, thus it will perform the following operations:

1. Recursively traverse the current node's left subtree
2. Recursively traverse the current node's right subtree
3. Visit the current node

If we traverse the example tree with post-order traversal, and print the visited node, the output will be: `ACBEFDIHG`

Here is how an example implementation of a post-order traversal of a binary tree using DFS would look like:


```{src='computer_science/dfs_postorder' caption='Post-order traversal of a tree using DFS'}
```

##### Reverse Traversals

These kinds of traversals are essentially the same of the ones we've already seen, but the right subtree is given priority over the left. Here are the operations, listed for reference.

Reverse Pre-Order:

1. Visit the current node
2. Recursively traverse the current node's right subtree
3. Recursively traverse the current node's left subtree

Reverse In-Order:

1. Recursively traverse the current node's right subtree
2. Visit the current node
3. Recursively traverse the current node's left subtree

Reverse Post-Order:

1. Recursively traverse the current node's right subtree
2. Recursively traverse the current node's left subtree
3. Visit the current node

The code will be omitted, since it is easy to infer how the code would look, given the previous examples.

#### Breadth-first search

Breadth-first search, or BFS, uses a concept that is opposite of the one in DFS (Depth-first search): instead of going as deep as possible inside the tree, this algorithm prefers exploring "in layers".

The root will be visited first, then all its children, after that all its nephews, etc...

![Order in which the nodes are visited during BFS](./images/computer_science/tree_traversal_bfs.svg){width=40%}

In the implementation shown here, the steps are the following:

1. Make a queue and enqueue the root
2. If the queue is not empty, take the first node, if it is empty, just stop
3. If such node has any children, enqueue them, in order
4. Visit the node
5. Go back to point 2

A possible implementation of a BFS algorithm could be the following:

```{src='computer_science/bfs' caption='Traversal of a tree using BFS'}
```

BFS is a great algorithm to solve mazes and find the shortest path between two nodes, making it a good choice for [Path finding](#pathfinding).
