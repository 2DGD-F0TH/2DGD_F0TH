\null\clearpage
Collision Detection and Reaction
=================================

When it comes to collision management, there are two main phases:

- **Collision Detection:** you find out which game objects collided with each other;
- **Collision Reaction:** you handle the physics behind the collision detected, making the game objects react to such collision.

Collisions don't only happen between game objects (two fighters hitting each other), but also between a character and the world (or they would end up just going through the ground).

In this section we'll talk about some ways you can detect and react to collisions.

Did it really collide?
-----------------------

First of all, we need to see how we can make sure that two objects really collide with each other.

### Collision Between Two Points

This is the simplest case: points are uni-dimensional objects, and the only way two points can collide is when they have the same coordinates.

An example algorithm would be the following:

~~~~
function point_collision(point A, point B):
    if A.x == B.x AND A.y == B.y:
        return True
    else
        return False
~~~~

A possible lazy/shorter version could be:

~~~~
function point_collision(point A, point B):
    return A.x == B.x AND A.y == B.y
~~~~

This algorithm consists in a constant number of operations, so it runs in O(1).

### Collision Between A Point and a Circle

Now a circle comes into the mix, a circle has two major characteristics: a **center** and a **radius**.

> A point is considered inside of a circle when the distance between the point and the center of the circle is *less than or equal* to the radius.

So we need a function that calculates the distance between two points, and then use it to define if a point is inside a circle.

An example could be the following:

~~~~
structure Circle:
    // Let's define a circle class/structure
    Point center;
    Integer radius

function distance(Point A, Point B):
    // Calculates the distance between two points
    return square_root((A.x + B.x)^2 + (A.y + B.y)^2)

function circle_point_collision(Circle A, Point B):
    if distance(A.center, B) <= A.radius:
        return True
    else:
        return False
~~~~

Again, the lazier version:

~~~~
structure Circle:
    // Let's define a circle class/structure
    Point center;
    Integer radius

function distance(Point A, Point B):
    // Calculates the distance between two points
    return square_root((A.x + B.x)^2 + (A.y + B.y)^2)

function circle_point_collision(Circle A, Point B):
    return distance(A.center, B) <= A.radius:
~~~~

Although slightly more heavy, computation-wise, this algorithm still runs in O(1).

### Collision Between Two Circles

Let's add another circle into the mix now, we can declare:

> Two circles are colliding when the distance between their centers is less or equal the sum of their radii

In pseudo code this would be:

~~~~
structure Circle:
    // Let's define a circle class/structure
    Point center;
    Integer radius

function distance(Point A, Point B):
    // Calculates the distance between two points
    return square_root((A.x + B.x)^2 + (A.y + B.y)^2)

function circle_circle_collision(Circle A, Circle B):
    if distance(A.center, B.center) <= A.radius + B.radius:
        return True
    else:
        return False
~~~~

The shorter version would be:

~~~~
structure Circle:
    // Let's define a circle class/structure
    Point center;
    Integer radius

function distance(Point A, Point B):
    // Calculates the distance between two points
    return square_root((A.x + B.x)^2 + (A.y + B.y)^2)

function circle_circle_collision(Circle A, Circle B):
    return distance(A.center, B.center) <= A.radius + B.radius:
~~~~

Again, this algorithm performs a number of operations that is constant, so it runs in O(1).


### Collision Between Two Axis-Aligned Rectangles (AABB)

<!-- TODO: Aka Axis-Aligned Bounding Box method -->

Finding out who hit what
------------------------

First of all, we need to find which game objects collided, and this can be easily one of the most expensive parts of our game, if not handled correctly.

We need to remember that each object (as good practices suggest) know only about themselves, they don't have "eyes" like us, that can see when another object is approaching them and thinking "I'm gonna collide". The only thing we can do it having "someone else" take care of checking for collisions.

As an example, we'll take the following situation:

![Example for collision detection](./images/collision_detection/collision_example.pdf){width=30%}

We can evidently see how circles 1 and 2 are colliding, but obviously our game won't just "know" without giving it a way to think about how two objects collide.

### The Brute Force Method

The simplest method is the so-called "brute force" method: you don't know which items may collide? Just try them all.

So if we consider a list of 7 game objects, we'll need to see if 1 collides with 2, 1 collides with 3, ..., 2 collides with 1, ...

An algorithm of this type could be the following:

~~~~
function is_collision(item A, item B):
    // Defines how two items collide (being circles, this could be a difference of radii)

items = [1, 2, 3, 4, 5, 6, 7]
colliding_items = []

for A in items:
    for B in items:
        if A is not B:
            // We avoid checking if an item collides with itself, for obvious reasons
            if is_collision(A, B):
                colliding_items.add((A, B))
~~~~

This algorithms runs in O(n^2^), because it checks every item with every other, even with itself.

In this example, the algorithm completes in 49 steps, but you can imagine how a game could slow down when there is an entire world to update (remember the collision detection, among with other updates and rendering/drawing, must happen in less than 16.67 and 33.33ms, so if you can save time, you totally should).

### Building Quad Trees

<!-- TODO: Easier on the CPU but harder to implement, every frame you build a quad tree
and use that to check on collisions -->

Collision Reaction
--------------------

### The Direction + Velocity Method
<!-- TODO: A-la mario 1, you get inside a block, and react according to where the character is going-->

### The "Before and After" Method
<!-- TODO: Snapshot before and after updating, react accordingly, allows for more advanced stuff -->

Some wilder stuff
-------------------

### The "Tile + Offset" Method
<!-- TODO: Useful for games like pacman, check the direction where you are going using the offset, if the next cell is a wall, react -->

Common Issues with Collision Detection
----------------------------------------

### The "Bullet Through Paper" problem

<!-- TODO: How a really small object at fast speeds can go through a thin wall without
the collision detection algorithm realizing it -->

### Precision Issues

<!-- TODO: Sometimes when sub-pixel precision is involved, there might be instances where the character stays "floating" for a single frame, giving a "spazzy" animation -->
