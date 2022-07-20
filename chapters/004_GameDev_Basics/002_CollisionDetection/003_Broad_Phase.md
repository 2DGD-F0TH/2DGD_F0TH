Broad-phase collision detection: is a collision even possible?
--------------------------------------------------------------

Now we need to find which game objects collided, and this can be easily one of the most expensive parts of our game, if not handled correctly.

This section will show how knowing which items will surely **not** collide can help us optimize our algorithms.

We need to remember that each object (as good practices suggest) know only about themselves, they don't have "eyes" like us, that can see when another object is approaching them and thinking "I'm gonna collide". The only thing we can do it having "someone else" take care of checking for collisions.

As an example, we'll take the following situation:

![Example for collision detection](./images/collision_detection/collision_example.svg){width=30%}

We can evidently see how circles 1 and 2 are colliding, but obviously our game won't just "know" without giving it a way to think about how two objects collide.

### The Brute Force Method {#brute_force}

The simplest method is the so-called "brute force" method: you don't know which items may collide? Just try them all.

So if we consider a list of 7 game objects, we'll need to see if 1 collides with 2, 1 collides with 3, ..., 2 collides with 1, ...

An algorithm of this type could be the following:

```{src='collisiondetection/brute_force' caption='Brute Force Method of collision search'}
```

This algorithms runs in O(n^2^), because it checks every item with every other, even with itself.

In this example, the algorithm completes in 49 steps, but you can imagine how a game could slow down when there is an entire world to update (remember the collision detection, among with other updates and rendering/drawing, must happen in less than 16.67 and 33.33ms, so if you can save time, you totally should).

### Building Quad Trees

A nice idea would be being able to limit the number of tests we perform, since the brute force method can get really expensive really quickly.

When building quad-trees, we are essentially dividing the screen in "quadrants" (and if necessary, such quadrants will be divided into sub-quadrants), detect which objects are in such quadrants and test collisions between objects that are inside of the same quadrant.

![Graphical example of a quad tree, overlaid on the reference image](./images/collision_detection/collision_quad_example.svg){width=40%}

And here below we can see how a quad tree would look, in its structure:

![A quad tree](./images/collision_detection/quad_tree.svg){width=50%}

The rules to follow in a quad tree are simple, both in filling and retrieval. When we are filling a quad tree:

- Each node starts by being inserted in the root;
- If the root is "full" (exceeds a set quantity of nodes), it "splits" into 4 sub-trees;
- If a node would fit in two quadrants (like #5), it gets put inside the parent of both quadrants.

When we are retrieving the nodes we will know that an object inside a certain node can collide only with the objects in the same nodes or in the subtree rooted at such node.

With the original brute force method, we will make at most 49 tests for 7 items (although it can be optimized), while with quad trees we will perform:

- 6 Tests against node 5 (5-1, 5-2, 5-3, 5-4, 5-6, 5-7);
- 1 Test against node 1 (1-2);
- 1 Test against node 2 (2-1);
- No tests against node 3, because it's on its own and there are no subtrees;
- No tests against node 4, for the same reason;
- 1 Test against node 6 (6-7);
- 1 Test against node 7 (7-6).

For a total of 10 tests, which can be further optimized by avoiding testing pairs of objects that have already been tested. But this is if we want to test all objects for collision against all other objects (thus it is a somewhat more optimized "brute force").

#### A more precise definition

To be more precise, quad-trees are part of the group of "spatial acceleration structures". They are structures that are usually used on top of other containers (like arrays) to accelerate or reduce the number of accesses.

For example, you may have an existing array and using pointers you can use a quad-tree to quickly refer to the place in memory a certain object is.

![Quad trees as spacial acceleration structures](./images/collision_detection/quad_tree_accelerator_1.svg){width=50%}

Redundancy will help us making things quicker and easier, adding a pointer from the underlying data structure back to the quad-tree will help us understanding where an object is positioned.

![Redundancy in quad-tree pointers](./images/collision_detection/quad_tree_accelerator_2.svg){width=50%}

#### Querying quad trees

Where quad trees shine is when we have an object and we want to check for collisions with any other object.

Using our "back pointer" we can refer back to the quad tree and severely limit the number of collision tests: any object will be able to collide only with its ancestors or descendants.

<!-- TODO: Make a step-by-step query of a quad-tree -->

{{placeholder}}

<!-- TODO: Easier on the CPU but harder to implement, you build a quad tree and use that to check on collisions: every frame you can use special moving operations to keep the Quad-tree up to date -->

### Building AABB-Trees

Another way to efficiently execute a broad-phase collision detection is by building trees containing Axis-Aligned Bounding Boxes.

The main idea is similar to what we've seen with binary search trees, mixed with the quad-trees we've just talked about: we are trying to keep track of objects that are close together (like Quad-Trees do) and when searching, we try to eliminate a good portion of data each time we descend the tree (similarly to binary search trees).

This is done by calculating a "cost function" every time we insert an object into the tree: our objective is making the cost as little as possible. An idea for the cost function could be the size of the rectangle (expressed by its perimeter, or just $width + height$).

Our example image, would be represented this way:

![How an AABB-tree would process our example image](./images/collision_detection/collision_aabb_example.svg){width=50%}

This can look a bit confusing, let's see how the tree would look like:

![How a possible AABB-tree structure would look like](./images/collision_detection/aabb_tree.svg){width=50%}

The performance of this tree is tightly related to its "balancing": differently from other types of "balanced trees", AABB-trees rely on how evenly each parent node is split by its children (instead of the usual "depth" metric). If an AABB-tree doesn't split evenly, the algorithm won't be able to "exclude" as many nodes on each iteration, thus degrading to a brute-force method (trying the given AABB against all other bounding boxes).

#### Querying AABB-trees

The idea behind this type of tree is making queries as fast as we can, and that can be done by checking on smaller rectangles on every iteration of our search algorithm. For instance we can find a list of possible colliding entities with a given bounding box in only a few tests (in our example).

Let's take for instance a circle "P" that is exactly between the points 3 and 4:

![Example of a search in an AABB-Tree](./images/collision_detection/aabb_tree_query.svg){width=50%}

First we do the root test, to see if it may collide with any of the 7 circles we have (if it was outside of the green rectangle, we would have finished already). Then we do the "left (cyan) child" test, in this case we're not colliding with the relative bounding box, so we keep going.:

![Querying an AABB-tree (1/3)](./images/collision_detection/aabb_tree_query_explained_1.svg){width=50%}

This way we excluded 1,2,6, and 7. We now do the "right (cyan) child" test, we're colliding with the relative bounding box, we continue on this branch.

![Querying an AABB-tree (2/3)](./images/collision_detection/aabb_tree_query_explained_2.svg){width=50%}

We do the "left (red) child" test, we're colliding with the relative bounding box, now we can do a narrow-phase collision detection with the leaves of this node (and in the meantime we also excluded 5).

![Querying an AABB-tree (3/3)](./images/collision_detection/aabb_tree_query_explained_3.svg){width=50%}


{{placeholder}}

<!-- TODO: A simple explanation of AABB trees to allow for account for proximity of objects -->

### Collision groups

{{placeholder}}

<!-- TODO: Collidable objects can be separated into groups which can only collide with each other (for instance bullets can collide with enemies, but not with walls since we may not be interested). This reduces the number of queries and makes for more generic code. -->

Other Collision Detection Methods
---------------------------------

### Calculating the position of tiles

When you are using tiles to build a level, being able to use quad trees or brute force methods to limit the number of collision checks inside your game may be harder than other methods.

Using a bit of math is probably the easiest and most efficient method to find out which collisions happened.

Let's take an example level:

![Example tile-based level](images/collision_detection/Tile_Calc_Example_Level_1.png){width=40%}

If a game entity is falling, like in the following example:

![Tile-based example: falling](images/collision_detection/Tile_Calc_Falling.png){width=40%}

Using the simple AABB collision detection, we will need to check only if the two lowest points of the sprite have collided with any tile in the level.

First of all let's consider a level as a 2-dimensional array of tiles and all the tiles have the same size, it is evident that we have two game entities that work with different measures: the character moves pixel-by-pixel, the ground instead uses tiles. We need something to make a conversion.

Assuming `TILE_WIDTH` and `TILE_HEIGHT` as the sizes of the single tiles, we'll have the following function:

```{src='collisiondetection/tile_conversion' caption='Converting player coordinates into tile coordinates'}
```

To know which tiles we need to check for collision, we just have to check the two red points (see the previous image), use the conversion function and then do a simple AABB check on them.

```{src='collisiondetection/tile_collision' caption='Tile-based collision detection'}
```

Considering that this algorithm calculates its own colliding tiles, we can state that its complexity is `O(n)` with `n` equal to the number of possibly colliding tiles calculated.

If an object is bigger than a single tile, like the following example:

![Example tile-based level with a bigger object](images/collision_detection/Tile_Calc_Example_Level_2.png){width=40%}

We will need to calculate a series of intermediate points (using the `TILE_WIDTH` and `TILE_HEIGHT` measures) that will be used for the test

![Tile-based example with a bigger object: falling](images/collision_detection/Tile_Calc_Falling_2.png){width=40%}

And using the same method the colliding tiles can be found without much more calculations than the previous algorithm, actually we can use exactly the same algorithm with a different list of points to test.
