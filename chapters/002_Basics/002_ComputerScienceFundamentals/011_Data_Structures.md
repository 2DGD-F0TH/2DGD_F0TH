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

{{placeholder}}

<!-- TODO: Talk about trees, with a special focus on binary trees -->

#### Depth-first Search

{{placeholder}}

<!-- TODO: Talk about DFS and what uses it has -->

##### Pre-order Traversal

{{placeholder}}

<!-- TODO: Node - Right - Left in Binary Trees -->

##### In-order Traversal

{{placeholder}}

<!-- TODO: Left - Node - Right in Binary Trees -->

##### Post-order Traversal

{{placeholder}}

<!-- TODO: Left - Right - Node in Binary Trees -->

#### Breadth-first search

{{placeholder}}

<!-- TODO: Search by layers -->
