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

Since numbers in computers can be **really** precise, a collision between two points may be a bit too precise, so it could prove useful to have a "buffer" around the point, so that we can say that the two points collided when they're **around the same place**.

<!-- TODO: Add point/point collision with buffer zone -->

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

This is one of the most used types of collision detection used in games: it's a bit more involved than other types of collision detection, but it's still computationally easy to perform. This is usually called the "Axis Aligned Bounding Box" collision detection, or AABB.

Let's start with a bit of theory. We have two squares:

![Example used in the AABB collision detection](./images/collision_detection/AABB1.pdf){width=30%}

To know if we may have a collision, we need to check if one of the sides is "inside" (that means between the top and bottom sides) of another rectangle:

![Top-Bottom Check](./images/collision_detection/AABB2.pdf){width=30%}

In this case we know that the "top side" of the second rectangle (highlighted in blue) has a `y` coordinate between the first rectangle's top and bottom sides' `y` coordinates (highlighted in red).

Though this is a necessary condition, this is not sufficient, since we may have a situation where this condition is satisfied, but the rectangles don't collide:

![Top-Bottom Check is not enough](./images/collision_detection/AABB3.pdf){width=30%}

So we need to check the other sides also, in a similar fashion:

![An example of a left-right check](./images/collision_detection/AABB4.pdf){width=30%}

This has to happen for all four sides of one of the rectangle.

Now we can try putting down a bit of code, we'll assume that rectangles are defined by their top-left corner (as usually happens) and their width and height:

~~~~
structure Point:
    // Rewritten as a memo
    Integer x
    Integer y

structure Rectangle:
    Point corner
    Integer width
    Integer height

function rect_rect_collision(Rectangle A, Rectangle B):
    if (A.corner.x < B.corner.x + B.width) AND
       (A.corner.x + A.width > B.corner.x) AND
       (A.corner.y < B.corner.y + B.height) AND
       (A.corner.y + A.height > A.corner.y):
        return True
    else:
        return False
~~~~

This complex conditional checks 4 things:

- The left side of rectangle A is **at the left** of the right side of rectangle B;
- The right side of rectangle A is **at the right** of the left side of rectangle B;
- The top side of rectangle A is **over** the bottom side of rectangle B;
- The bottom side of rectangle A is **underneath** the top side of rectangle B.

If all four checks are true, then a collision happened.

The best way to understand this algorithm properly is to test it by hand and convince yourself that it works.

This is a very light algorithm but can quickly become heavy on the CPU when there are many objects to check for collision. We'll see later how to limit the number of checks and make collision detection an operation that is not as heavy on our precious CPU cycles.

### Line/Point Collision

We can represent a segment by using its two extreme points, which proves to be a quite inexpensive way to represent a line (it's just two points). Now how do we know if a point is colliding with a line?

To know if a point is colliding with a line we need... Triangles!

Every triangle can be represented with 3 points, and there is a really useful theorem that we can make use of:

> The sum of the lengths of any two sides must be greater than, or equal, to the length of the remaining side.

So, given a triangle ABC:

![Example of the triangle inequality theorem](./images/collision_detection/triangle_ineq1.pdf){width=30%}

We get the following 3 inequalities:

$$ \overline{AB} + \overline{BC} \leq \overline{AC} $$
$$ \overline{AC} + \overline{BC} \leq \overline{AB} $$
$$ \overline{AB} + \overline{AC} \leq \overline{BC} $$

What is more interesting to us is that when the one of the vertices of the triangle is **on** its opposite side, the triangle degenerates:

![Example of a degenerate triangle](./images/collision_detection/triangle_ineq2.pdf){width=30%}

And the theorem degenerates too, to the following:

$$ \overline{AC} + \overline{BC} = \overline{AB}$$

So we can calculate the distance between the point and each of the two extremes of the line and we know that when the sum of such distances is equal to the length of the line, the point will be colliding with the line.

In code, it would look something like the following:

~~~~
structure Point:
    Integer x
    Integer y

structure Line:
    Point A
    Point B

function distance(Point A, Point B):
    // Calculates the distance between two points
    return square_root((A.x + B.x)^2 + (A.y + B.y)^2)

function line_point_collision(Point pt, Line ln):
    // First, let's calculate the length of the line
    length = distance(ln.A, ln.B)
    // Now let's calculate the distance between the point pt
    // and the point "A" of the line
    pt_a = distance(ln.A, pt)
    // Same Goes for the distance between pt and "B"
    pt_b = distance(ln.B, pt)
    // Now for the detection
    if (pt_a + pt_b == length):
        return True
    else:
        return False
~~~~

It could prove useful to put a "buffer zone" in here too, so that the collision detection doesn't result too jerky and precise.

<!-- TODO: Line vs Point with buffer zone -->

### Line/Circle Collision

As in the previous paragraph, we memorize a line as a pair of Points, so checking if the circle collides with either end of the line is easy, using the Point/Circle collision algorithm.

~~~~
structure Point:
    Integer x
    Integer y

structure Line:
    Point A
    Point B

structure Circle:
    Point center
    Integer radius

...

function line_circle_collision(Circle circle, Line line):
    collides_A = circle_point_collision(circle, line.A)
    collides_B = circle_point_collision(circle, line.B)
    if (collides_A OR collides_B):
        return True
    ...
~~~~

Now our next objective is finding the closest point **on the line** to the center of our circle. The details and demonstrations on the math behind this will be spared, just know the following:

Given a line $\overline{AB}$ between points $A=(x_1,y_1)$ and $B=(x_2,y_2)$ and a point $P=(x_k, y_k)$, the point on the line closest to P has coordinates:

$$x=x_1 + u \cdot (x_2 - x_1)$$
$$y=y_1 + u \cdot (y_2 - y_1)$$

With:

$$ u = \frac{(x_k - x_1)\cdot(x_2 - x_1) + (y_k-y_1)\cdot(y_2-y_1)}{||B-A||^2}$$

That's a lot of math!

We need to be careful though, cause this formula gives us the point for an *infinite* line, so the point we find could be outside of our line. We will use the line/point algorithm to check for that.

After we made sure the point is on the line, we can measure the distance between such point and the center of our circle, if such distance is less than the radius, we have a hit! (Or just apply the circle/point collision algorithm again).

The final algorithm should look something like this:

~~~~
structure Point:
    Integer x
    Integer y

structure Line:
    Point A
    Point B

structure Circle:
    Point center
    Integer radius

function distance(Point A, Point B):
    // Calculates the distance between two points
    return square_root((A.x + B.x)^2 + (A.y + B.y)^2)

function line_point_collision:
    ...

function circle_point_collision:
    ...

function line_circle_collision(Circle circle, Line line):
    // We check the ends first
    collides_A = circle_point_collision(circle, line.A)
    collides_B = circle_point_collision(circle, line.B)
    if (collides_A OR collides_B):
        return True
    // We pre-calculate "u", we'll use some variables for readability
    x1 = line.A.x
    x2 = line.B.x
    xk = circle.center.x
    y1 = line.A.y
    y2 = line.B.y
    yk = circle.center.y
    u = ((xk - x1) * (x2 - x1) + (yk - y1) * (y2 - y1))/(distance(line.A, line.B))^2
    // Now let's calculate the x and y coordinates
    x = x1 + u * (x2 - x1)
    y = y1 + u * (y2 - y1)
    // "Reuse", we'll use some older functions, let's create a point, with the coordinates we found
    P = Point(x,y)
    // Let's check if the "closest point" we found is on the line
    if (line_point_collision(line, P)) == False:
        // If the point is outside the line, we return false, because the ends have already been checked against collisions
        return False
    else:
        // Let's Reuse the Point/Circle Algorithm
        return circle_point_collision(circle, P)
~~~~

### Circle/Rectangle Collision

Circle/Rectangle collision is performed by executing a Circle/Line algorithm with the circle and the closest side of the rectangle as arguments.

<!-- TODO: Like Line/Circle, but with the closest rectangle edge -->

### Line/Line Collision

<!-- TODO: Can be useful for line-based puzzle games, like the untangle puzzle -->

### Line/Rectangle Collision

<!-- TODO: Just a Line/Line collision done 4 times: one for each side of the rectangle -->

### Pixel-Perfect collision

Pixel perfect collision is the most precise type of collision detection, but it's also by far the slowest.

The usual way to perform collision detection is using **bitmasks** which are 1-bit per pixel representation of the sprites (white is usually considered a "1" while black is considered a "0").

A logic "AND" operation is performed, pixel-by-pixel, on the bitmasks; with the sprite position taken in consideration, as soon as the first AND returns a "True" a collision occurred.

~~~~
structure Color:
    Integer colorData
    Boolean isWhite()

structure Bitmask:
    Color[] data
    Color getColor(x, y)

structure Sprite:
    Bitmask bitmask
    Integer x
    Integer y
    Integer width
    Integer height

function pixel_perfect_collision(Sprite A, Sprite B):
    // Calculate the intersecting rectangle to limit checks
    x1 = max(A.x, B.x)
    x2 = min((A.x + A.width), (B.x + B.width))

    y1 = max(A.y, B.y)
    y2 = min((A.y + A.height), (B.y + B.height))

    // For each pixes in the intersecting rectangle, let's check
    for each y from y1 to y2:
        for each x from x1 to x2:
            a = A.bitmask.getColor(x - A.x, y - A.y)
            b = B.bitmask.getColor(x - B.x, y - B.y)

            if (a.isWhite() AND b.isWhite()):
                return True

    // If no collision is occurred by the end of the checking, we're safe
    return False
~~~~

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
