Generic Programming
-------------------

Sometimes it may be necessary (mostly in the case of containers) to have the same kind of code to work on different data types, which means that we need to **abstract types into variables** and be able to code accounting for such types.

**Generic Programming** is a blanket-term that defines a style of computer programming where algorithms are written in terms of "to be specified later" data types, this usually applies to languages that make use of *[static typing~\[g\]~](#gl_statictyping)*.

Data Structures
---------------

This section is dedicated to give some basic explanation of some advanced containers and data structures that are used in computer science, allowing us to make an informed choice when we want to implement some even more advanced containers in the future.

Where possible, we will include big-O performance counters for the basic functions of: adding/removing and item at the beginning, adding/removing an item at the end, adding/removing an item in an arbitrary position and indexing at a certain position.

This section is in no way exhaustive, but should be enough to make an informed decision on what containers to use for our components, according to necessities.

:::: note ::::
This section will be purely theoretical and many data structures will have no code blocks, this is because implementations vary wildly between programming languages and some of these structures are integrated in such languages.
::::::::::::::

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

### Dynamic Arrays

In many languages, arrays are sized statically, with a size decided at compile time. This severely limits the array's usefulness.

Dynamic Arrays are a wrapper around arrays, allowing it to extend its size when needed. This usually entails some additional operations when inserting or deleting an item.

![Dynamic Arrays Reference Image](./images/computer_science/dynamic_arrays.svg){width=40%}

#### Performance Analysis

Indexing an item is immediate, since arrays allow to natively index themselves.

Inserting an item at the beginning is a heavy task, since it requires either moving all the present items or rebuilding the internal native array. Such operations require copying or moving each element, giving us a time complexity averaging on `O(n)`.

![Adding an element at the beginning of a Dynamic Array](./images/computer_science/dynamic_arrays_insert_head.svg){width=90%}

Inserting an item at the end, if we keep a pointer to the last item inserted, is an operation that usually happens immediately (time complexity `O(1)`), but when the array is full, we need to instantiate a new native array (usually double the size of the current one) and copy all elements inside the new array (operation that has time complexity of `O(n)`). Since the number of `O(1)` operations outweighs by a long shot the number of `O(n)` operations, it's possible to demonstrate that in the long run appending an item at the end of a dynamic array has a time complexity averaging around `O(1)`.

![Adding an element at the end of a Dynamic Array](./images/computer_science/dynamic_arrays_insert_tail.svg){width=75%}

Inserting an item in an arbitrary position, much like inserting an item at the beginning requires moving some items further into the array, potentially all of them (when the arbitrary position is the beginning of the array), thus giving us a time complexity of `O(n)`. Such operation could trigger an array resize, which has no real influence on the estimate.

![Adding an element at an arbitrary position of a Dynamic Array](./images/computer_science/dynamic_arrays_insert_arbitrary.svg){width=90%}

Some implementations of the Dynamic Arrays try to save space when the number of items goes lower than $\frac{1}{4}$ of the array capacity during a deletion, the internal array is rebuilt with half the size. Such operation has a time complexity of `O(n)`.

Some other implementations use a $\frac{1}{4}$/$\frac{3}{4}$ rule, halving the array capacity when the item deletion brings the number of items lower than $\frac{1}{4}$ of the array and doubling it when an insertion makes the number of elements higher than $\frac{3}{4}$ of the array capacity.

:::: note ::::
Not all programming languages have native support for arrays, for instance Python normally uses lists (although it supports arrays via the `array` standard library).
::::::::::::::

| Operation                  | Average Cost           |
| :---------:                | :-----:                |
| Indexing                   | O(1)                   |
| Insert/Delete At Beginning | O(n)                   |
| Insert/Delete At End       | O(1) amortized         |
| Insert/Delete at position  | O(n)                   |

Table: Performance table for Dynamic Arrays

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Dynamic Array

**When To Use it**          All situations that require direct indexing of a container, but insertions and removals are not extremely common, and usually take the form of "push back" (insertion at the end)

**Advantages**              Direct Indexing, Fast iteration through all the elements, given by the fact that arrays are stored compact in memory, fast appending.

**Disadvantages**           Slow insertions in arbitrary positions and at the head of the array.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Dynamic Arrays

### Linked Lists

Linked Lists are a data structure composed by "nodes", each node contains data and a reference to the next node in the linked list. Differently from arrays, nodes may not be contiguous in memory, which makes indexing problematic.

![Linked List Reference Image](./images/computer_science/linked_list_reference.svg){width=60%}

Some implementations feature a pointer to the last element of the list, to make appending items at the end easier and quicker.

![Double-Ended Linked List Reference Image](./images/computer_science/de_linked_list_reference.svg){width=60%}

#### Performance Analysis

Since we only have a handler on the first node, indexing requires us to scan all the elements until we reach the one that was asked for. This operation has a potential time complexity of `O(n)`.

Inserting an item at the beginning is immediate, we just need to create a new node, make it point at the current head of the list and then update our "handle" to point at the newly created node. The number of operations is independent of how many data we already have, so the time complexity is `O(1)`.

![Inserting a new node at the beginning of a linked list](./images/computer_science/linked_list_insert_head.svg){width=90%}

Appending an item at the end has a time complexity that varies depending on the chosen implementation: if the list has a reference to the final node, we just need to create a new node, update the final node's reference (usually called "next") to point at the new node and then update the reference to the final node to point at the newly created node (time complexity `O(1)`). If our queue doesn't have such reference, we will need to scan the whole list to find the final node (time complexity `O(n)`).

![Inserting a new node at the end of a (double-ended) linked list](./images/computer_science/de_linked_list_insert_tail.svg){width=90%}

Inserting at an arbitrary position requires us to scan the list until we find the position that we want, after that we just need to split and rebuild the references correctly, which is a fast operation.

![Inserting a new node at an arbitrary position in a (double-ended) linked list](./images/computer_science/de_linked_list_insert_arbitrary.svg){width=90%}

| Operation                  | Average Cost                         |
| :---------:                | :-----:                              |
| Indexing                   | O(n)                                 |
| Insert/Delete At Beginning | O(1)                                 |
| Insert/Delete At End       | O(1) for double-ended, o(n) otherwise|
| Insert/Delete at position  | time to search + O(1)                |

Table: Performance table for Linked Lists

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Linked List

**When To Use it**          All situations that require quick insertions/removals, either on the head or the tail (used as stacks or queues).

**Advantages**              Very fast insertions/removals, quite fast iteration through all the elements.

**Disadvantages**           Slow indexing at an arbitrary position. Sorting can be complex.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Linked Lists

### Doubly-Linked Lists

A doubly-linked list is a variation of a linked list where each node not only has a reference to its successor, but also a reference to its predecessor. This allows for easy processing of the list in reverse, without having to create algorithms that entail a huge overhead.

All the operations of insertion, indexing and deletion are performed in a similar fashion to the classic singly-linked list we saw earlier, just with an additional pointer to account for.

![Doubly Linked List Reference Image](./images/computer_science/doubly_linked_list_reference.svg){width=60%}

| Operation                  | Average Cost          |
| :---------:                | :-----:               |
| Indexing                   | O(n)                  |
| Insert/Delete At Beginning | O(1)                  |
| Insert/Delete At End       | O(1)                  |
| Insert/Delete at position  | time to search + O(1) |

Table: Performance table for Doubly-Linked Lists

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Doubly-Linked List

**When To Use it**          All situations that require quick insertions/removals, either on the head or the tail (stacks or queues) or iterating through an entire list, forwards or backwards.

**Advantages**              Very fast insertions/removals, quite fast iteration through all the elements. Possibility of easily iterating the list in reverse order.

**Disadvantages**           Slow indexing at an arbitrary position. Sorting can be complex.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Linked Lists

### Hash Tables

Hash Tables are a good way to store **unordered data** that can be referred by a "key". These structures have different names, like "maps", "dictionaries" or "hash maps".

The idea behind a hash map is having a key subject to a *[hash function~\[g\]~](#gl_hashfunc)* that will decide where the item will be positioned in the internal structure.

![Hash Table Reference Image (Hash Table with Buckets)](./images/computer_science/hashtable_reference.svg){width=40%}

The simplest way to implement a hash table is using an "array with buckets": an array where each cell has a reference to a linked list.

On average, finding an item requires passing the key through the hash function, such hash function will tell us where the item is in our internal structure immediately. Thus giving a time complexity of $O(1)$.

Inserting has more or less the same performance, the key gets worked through the hash function, deciding which linked list will be used to store the item.

Deletion works in the same fashion, passing the key through the hash function and then deleting the value; giving a time complexity of $O(1)$

| Operation  | Average Cost   |
| :---------:| :-----:        |
| Searching  | O(1)           |
| Insert     | O(1)           |
| Delete     | O(1)           |

Table: Performance table for Hash Tables

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Hash Table

**When To Use it**          All situations that require accessing an element by a well-defined key quickly. Building unordered data sets.

**Advantages**              Fast insertions/removals, direct indexing (in absence of hash collisions) by key.

**Disadvantages**           In case of a bad hashing function, it reverts to the performance of a linked list, cannot be ordered.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Hash Tables

### Binary Search Trees (BST) {#bst}

Binary search trees, sometimes called "ordered trees" are a container that have an "order relation" between their own elements.

![Binary Search Tree Reference](./images/computer_science/BST.svg){width=60%}

The order relation allows us to have a tree that is able to distinguish between "bigger" and "smaller" values, thus making search really fast at the price of a tiny slowdown in insertion and deletion.

Searching in a BST is easy, starting from the root, we check if the current node is the searched value; if it isn't we compare the current node's value with the searched value.

If the searched value is greater, we search on the right child. If it is smaller, we continue our search on the left child.

Recursively executing this algorithm will lead us to find the node, if present. Such algorithm has a $O(log(n))$ time complexity.

In a similar fashion, insertion will recursively check subtrees until the right spot of the value is found. The insertion operation has the same time complexity as searching: $O(log(n))$.

Deletion is a bit more conceptually complex, since it's necessary to maintain the ordering of the nodes. Such operation has a time complexity of $O(log(n))$.

| Operation  | Average Cost        |
| :---------:| :-----:             |
| Searching  | O(log(n))           |
| Insert     | O(log(n))           |
| Delete     | O(log(n))           |

Table: Performance table for Binary Search Trees

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Binary Search Tree

**When To Use it**          Situations that require good overall performance and requires fast search times.

**Advantages**              Good insertion and removal times, searching on this structure is fast.

**Disadvantages**           Given the nature of the data structure, there is no direct indexing, nor ordering.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Binary Search Trees

### Heaps

Heaps are a tree-based data structure where we struggle to keep a so-called "heap property". The heap property defines the type of heap that we are using:

- **Max-Heap:** For each node `N` and its parent node `P`, we'll always have that the value of `P` is always greater or equal than the value of `N`;
- **Min-Heap:** For each node `N` and its parent node `P`, we'll always have that the value of `P` is always less or equal than the value of `N`;

![Heap Reference Image (Min-Heap)](./images/computer_science/heap_reference.svg){width=60%}

Heaps are one of the maximally efficient implementation of priority queues, since the highest (or lowest) priority item is stored in the root and can be found in constant time.

| Operation      | Average Cost                                               |
| :------:       | :-----------------:                                        |
| Find Minimum   | $O(1)$ to $O(log(n))$, depending on the implementation     |
| Remove Minimum | $O(log(n))$                                                |
| Insert         | $\Theta(1)$ to $O(log(n))$ depending on the implementation |

Table: Performance table for Heaps

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Heap

**When To Use it**          All situations where you require to find and/or extract the minimum or maximum value in a container quickly; like priority queues.

**Advantages**              Good general time complexity, maximum performance when used as priority queues.

**Disadvantages**           No inherent ordering, there are better solutions for general use.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Heaps

### Stacks

Stacks are a particular data structure, they have a limited way of working: you can only put or remove items on top of the stack, plus being able to "peek" on top of the stack.

![How a stack works](./images/computer_science/stack.svg){width=30%}

Stacks are LIFO (Last in - First Out) data structures, and can be implemented with both a linked list or a cleverly-indexed array.

Depending on the single implementation, the operation used to "pop" an item from the stack will also return the element, ready to be used in an upcoming computation.

![Array and linked list implementations of a stack](./images/computer_science/stack_implementation.svg){width=40%}

### Queues

Queues are the exact opposite of stacks, they are FIFO (First in - First Out) data structures: you can put items on the back of the queue, while you can remove from the head of the queue.

![How a queue works](./images/computer_science/queue.svg){width=50%}

Depending on the single implementation, the operation used to "dequeue" an item from the queue will also return the element just removed, ready to be used in an upcoming computation.

As with stacks, queues leverage limitations in their way of working for greater control over the structure itself. Usually queues are implemented via linked lists, but can also be implemented via arrays, using multiple indexes and index-wrapping when iterating.

![Array and linked list implementation of a queue](./images/computer_science/queue_implementation.svg){width=40%}

### Circular Queues

Circular Queues are a particular kind of queues that are infinitely iterable, every time an iterator goes after the last element in the queue, it will wrap around to the beginning.

![How a circular queue works](./images/computer_science/circular_queue.svg){width=70%}

Circular Queues can be implemented via linked lists or cleverly indexed arrays, with all the advantages and disadvantages that such structures entail.

![Array and linked list implementation of a circular queue](./images/computer_science/circular_queue_implementation.svg){width=50%}

