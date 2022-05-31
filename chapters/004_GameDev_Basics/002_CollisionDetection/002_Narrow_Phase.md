Narrow-Phase Collision Detection: did it really collide?
-------------------------------------------------------

First of all, we need to see how we can make sure that two objects really collide with each other.

Sometimes this presents a (quite common) problem when it comes to precision: computers have no knowledge of infinity (due to their finiteness, see [computers are (not) precise](#precision_issues)). This means that we may need to give some leeway and define an "acceptable error" in our calculations, thus we will create a "small enough value" (which in math is represented by the greek letter "epsilon": $\epsilon$) and change our algorithms accordingly.

### Collision Between Two Points

This is the simplest case: points are mono-dimensional objects, and the only way two points can collide is when they have the same coordinates.

An example algorithm would be the following:

```{src='collisiondetection/point_to_point' caption='Point to point collision detection'}
```

A possible lazy/shorter version could be:

```{src='collisiondetection/point_to_point_lazy' caption='Shortened version of a point to point collision detection'}
```

This algorithm consists in a constant number of operations, so it runs in O(1).

Since numbers in computers can be **really** precise, a collision between two points may be a bit too precise, so it could prove useful to have a "buffer" around the point, so that we can say that the two points collided when they're **around the same place**.

In this case, it may prove to be a lot more useful to do a [point vs circle](#point_circle) detection, or even a [circle vs circle](#circle_circle) collision detection, in that case the "radius" would be the "approximation" of a point.

If instead you want to use a different method that doesn't involve square roots, you can use epsilon values to have an approximation of the collision. In this case the collision area won't be round, but square.

```{src='collisiondetection/point_to_point_epsilon' caption='Point to point collision detection with epsilon values'}
```

### Collision Between A Point and a Circle {#point_circle}

Now a circle comes into the mix, a circle has two major characteristics: a **center** and a **radius**.

![Reference image for Point-Circle Collision detection](./images/collision_detection/point_circle.svg){width=40%}

We can see that the distance between the center of a circle and our point can be expressed with a formula:

$$d = r + x$$

Where `r` is the circle radius and `x` is the difference of the distance between the center of the circle and the point (which can be negative):

$$x = d - r$$

The point is inside the circle when $x \leq 0$, which means:

$$x \leq 0 \Leftrightarrow d - r \leq 0 \Leftrightarrow d \leq r$$

We can express this in a few words:

> A point is considered inside of a circle when the distance between the point and the center of the circle is *less than or equal* to the radius.

So we need a function that calculates the distance between two points, and then use it to define if a point is inside a circle.

An example could be the following:

```{src='collisiondetection/point_circle' caption='Point to circle collision detection'}
```

Again, the lazier version:

```{src='collisiondetection/point_circle_lazy' caption='Shorter version of a point to circle collision detection'}
```

Although slightly more heavy, computation-wise, this algorithm still runs in O(1).

### Collision Between Two Circles {#circle_circle}

Let's add another circle into the mix now, and think in more or less the same way as before:

![Reference image for Circle-Circle collision detection](./images/collision_detection/circle_circle.svg){width=50%}

We can see the distance between the center of the circles as expressed with the following formula:

$$ d = r_1 + x + r_2 $$

Where $r_1$ and $r_2$ are the radii, and $x$ is defined as follows:

$$ x = d - (r_1 + r_2) $$

As before, our $x$ can be negative, which means that the circles are colliding if $x \leq 0$, which means:

$$ x \leq 0 \Leftrightarrow d - (r_1 + r_2) \leq 0 \Leftrightarrow d \leq r_1 + r_2$$

We can express the concept in words again:

> Two circles are colliding when the distance between their centers is less or equal the sum of their radii

In pseudo code this would be:

```{src='collisiondetection/circle_circle' caption='Circle to Circle Collision Detection'}
```

The shorter version would be:

```{src='collisiondetection/circle_circle_lazy' caption='Shorter Version of a Circle to Circle Collision Detection'}
```

Again, this algorithm performs a number of operations that is constant, so it runs in O(1).


### Collision Between Two Axis-Aligned Rectangles (AABB) {#AABB}

This is one of the most used types of collision detection used in games: it's a bit more involved than other types of collision detection, but it's still computationally easy to perform. This is usually called the "Axis Aligned Bounding Box" collision detection, or AABB.

Let's start with a bit of theory. We have two squares:

![Example used in the AABB collision detection](./images/collision_detection/AABB1.svg){width=30%}

To know if we may have a collision, we need to check if one of the sides is "inside" (that means between the top and bottom sides) of another rectangle:

![Top-Bottom Check](./images/collision_detection/AABB2.svg){width=30%}

In this case we know that the "top side" of the second rectangle (highlighted in blue) has a `y` coordinate between the first rectangle's top and bottom sides' `y` coordinates (highlighted in red).

Though this is a necessary condition, this is not sufficient, since we may have a situation where this condition is satisfied, but the rectangles don't collide:

![Top-Bottom Check is not enough](./images/collision_detection/AABB3.svg){width=30%}

So we need to check the other sides also, in a similar fashion:

![An example of a left-right check](./images/collision_detection/AABB4.svg){width=30%}

This has to happen for all four sides of one of the rectangle.

Now we can try putting down a bit of code, we'll assume that rectangles are defined by their top-left corner (as usually happens) and their width and height:

```{src='collisiondetection/AABB' caption='Axis-Aligned Bounding Box Collision Detection'}
```

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

![Example of the triangle inequality theorem](./images/collision_detection/triangle_ineq1.svg){width=30%}

We get the following 3 inequalities:

$$ \overline{AB} + \overline{BC} \leq \overline{AC} $$
$$ \overline{AC} + \overline{BC} \leq \overline{AB} $$
$$ \overline{AB} + \overline{AC} \leq \overline{BC} $$

What is more interesting to us is that when the one of the vertices of the triangle is **on** its opposite side, the triangle degenerates:

![Example of a degenerate triangle](./images/collision_detection/triangle_ineq2.svg){width=30%}

And the theorem degenerates too, to the following:

$$ \overline{AC} + \overline{BC} = \overline{AB}$$

So we can calculate the distance between the point and each of the two extremes of the line and we know that when the sum of such distances is equal to the length of the line, the point will be colliding with the line.

In code, it would look something like the following:

```{src='collisiondetection/line_point' caption='Line to Point Collision detection'}
```

It could prove useful to put a "buffer zone" in here too, so that the collision detection doesn't result too jerky and precise. In that case you may want to take a look at the [line vs circle](#line_circle) algorithm, in that case the radius would be the "approximation" of the point.

### Line/Circle Collision {#line_circle}

As in the previous paragraph, we memorize a line as a pair of Points, so checking if the circle collides with either end of the line is easy, using the Point/Circle collision algorithm.

```{src='collisiondetection/line_circle_partial' caption='Partial Implementation of a Line to Circle Collision Detection'}
```

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

```{src='collisiondetection/line_circle' caption='Line to circle collision detection'}
```

### Point/Rectangle Collision {#point_rectangle}

If we want to see if a point collides with a rectangle is really easy, we just need to check if the point's coordinates are inside the rectangle.

```{src='collisiondetection/point_rectangle' caption='Point/Rectangle collision detection'}
```

### Point/Triangle Collision

A possible way to define if a point is inside a triangle, we can use a bit of geometry.

We can use *Heron's formula* to calculate the area of the original triangle, and compare it with the sum of the areas created by the 3 triangles made from 2 points of the original triangle and the point we are testing.

![Point/Triangle Collision Detection: division into sub-triangles](./images/collision_detection/point_triangle.svg){width=30%}

If the sum of the 3 areas (represented in different colors in the figure) equals to the original calculated area, then we know that the point is inside the triangle.

Let's see the code:

```{src='collisiondetection/point_triangle' caption='Point/Triangle Collision Detection'}
```

Let's see how we can change the algorithm to accomodate for some leeway, since the we may be requiring too much precision from our algorithms. We can do that by using epsilon values.

Our main test is that the sum of the area of the 3 triangles we create ($A_1,A_2,A_3$) is equal to the area of the original triangle ($A_0$), in math terms:

$$ A_1 + A_2 + A_3 = A_0$$

We can also rewrite such equation this way:

$$ A_1 + A_2 + A_3 - A_0 = 0$$

Due to possible precision issues we know that there are some values where the equation above is not true, so we choose a "low enough error" that we are willing to accept, for example $\epsilon = 0.0001$, and use this test instead:

$$ | A_1 + A_2 + A_3 - A_0 | < \epsilon$$

Which can be expanded (if you want) to

$$ -\epsilon < A_1 + A_2 + A_3 - A_0 < \epsilon $$

The code wouldn't change much, but for sake of clarity, here it is:

```{src='collisiondetection/point_triangle_epsilon' caption='Point/Triangle Collision Detection with epsilon'}
```

### Circle/Rectangle Collision

First of all we need to identify which side of the rectangle we should test against, so if the centre of the circle is to the right of the rectangle, we will test against the right edge of the rectangle, if it's above we'll test against the top edge and so on...

After that, we just perform some math on the distances and calculated values to detect if the circle collides with the rectangle.

```{src='collisiondetection/rectangle_circle' caption='Rectangle to Circle Collision Detection'}
```

### Line/Line Collision

Line/Line collision is quite simple to implement once you know the inner workings of geometry, but first we need to explain the thought behind this algorithm, so... **math warning!!**

Let's look at the following image:

![Example image for line/line collision](./images/collision_detection/line_line.png){width=60%}

A generic point $P_a$ of line A can be represented with the following formula:

$$ P_a = P_1 + u_a \cdot (P_2 - P_1) $$

which translates into the coordinate-based equations:

$$
\begin{cases}
x_a = x_1 + u_a \cdot (x_2 - x_1)\\
y_a = y_1 + u_a \cdot (x_2 - x_1)
\end{cases}
$$

This makes us understand that any point of line A can be represented by its starting point $P_1$, plus a certain fraction (represented by $u_a$) of the vector represented by $P_2 - P_1$.

This also means that $0 \leq u_a \leq 1$, else the point won't be on the segment.

In the same way, a generic point $P_b$ of line B can be represented with:

$$ P_b = P_3 + u_b \cdot (P_4 - P_3) $$

which becomes:

$$
\begin{cases}
x_b = x_3 + u_b \cdot (x_4 - x_3)\\
y_b = y_3 + u_b \cdot (x_4 - x_3)
\end{cases}
$$

The two lines will collide when $P_a = P_b$, so we get the following equations:

$$
\begin{cases}
x_1 + u_a \cdot (x_2 - x_1) = x_3 + u_b \cdot (x_4 - x_3)\\
y_1 + u_a \cdot (y_2 - y_1) = y_3 + u_b \cdot (y_4 - y_3)
\end{cases}
$$

That need to be solved in the $u_a$ and $u_b$ variables.

The result is:

$$
\begin{cases}
u_a = \frac{(x_4 - x_3) \cdot (y_1 - y_3) - (y_4 - y_3) \cdot (x_1 - x_3)}{(y_4 - y_3) \cdot (x_2 - x_1) - (x_4 - x_3) \cdot (y_2 - y_1)}\\
u_b = \frac{(x_2 - x_1) \cdot (y_1 - y_3) - (y_2 - y_1) \cdot (x_1 - x_3)}{(y_4 - y_3) \cdot (x_2 - x_1) - (x_4 - x_3) \cdot (y_2 - y_1)}
\end{cases}
$$

Substituting either of the results in the corresponding equation for the line will give us the intersection point (which may be useful for some particle effects).

Now some notes on our solution:

- If the denominator for the equations for $u_a$ and $u_b$ equals to zero, the two lines are parallel
- If both the numerator and denominator for $u_a$ and $u_b$ are equal to zero, the two lines are coincident
- If both $0 \leq u_a \leq 1$ and $0 \leq u_b \leq 1$ then the two segments collide.

Now we can translate all this math into code:

```{src='collisiondetection/line_line' caption='Implementation of the line/line collision detection'}
```

This collision detection algorithm can be useful for line-based puzzle games, line the untangle puzzle.

### Line/Rectangle Collision

Given the previous explanation about the Line/Line collision detection, it's quite easy to build a Line/Rectangle algorithm; distinguishing the cases where we want to account for a segment being completely inside of a rectangle or not.

```{src='collisiondetection/line_rectangle' caption='Implementation of the line/rectangle collision detection'}
```

This can prove useful to test for "line of sight" inside an AI algorithm.

### Point/Polygon Collision

Here we are, the most complex matter when it comes to narrow-phase collision detection: detecting collisions between arbitrary convex polygons.

::: note :::
In this book we will focus on convex polygons "without holes", which is the most common situation you'll find yourself in.
:::::::::::::::

First of all, we will start by talking about some theorems and requirements that will help us on the way to build a "polygon vs polygon" collision detection algorithm.

#### Jordan Curve Theorem

Let's imagine a plane, like our 2D screen: if we draw a non-self-intersecting, continuous loop in the plane we obtain a *Jordan Curve*. This curve separates the plane in two distinct regions: the "inside" and the "outside".

![Example of a Jordan Curve](./images/collision_detection/jordan_curve_1.svg){width=50%}

Any non-self-intersecting polygon (be it convex or non-convex) can be seen as a Jordan curve, this means that we can easily identify (programmatically) if a point is inside or outside the polygon. At least in the "convex" case.

Let's take a convex polygon, and a point inside such polygon: we can see that if we choose a point outside the polygon (non-colliding) we can strike a line between the "inside point" and the chosen point, and such line will intersect one of the polygon's edges. This gives us an idea on how to check for "point vs. polygon".

![A simple case where a point is outside the polygon](./images/collision_detection/jordan_curve_2.svg){width=50%}

This doesn't happen if the point is inside the polygon, obviously:

![A simple case where a point is inside the polygon](./images/collision_detection/jordan_curve_3.svg){width=40%}

This is all well and good, but we have two problems on hand:

- Finding a point inside the polygon;
- We have a non-convex polygon;

Let's leave the first problem aside, since talking about it may end up being confusing and just empty talk (or writing, being this a book), and let's focus on the second problem.

If we have a non-convex polygon, we may end up with a line that intersects the polygon's perimeter even if the point is colliding:

![How a non-convex polygon makes everything harder](./images/collision_detection/jordan_curve_4.svg){width=50%}

Here we call $P$ the "point inside the polygon" while $Q_1$ and $Q_2$ are the points we are testing: as we can see $Q_2$ triggers our "non-colliding" test even though it is inside the polygon.

Can you see what can help us solving this issue? I'm sure you have a *number* of ideas in mind, we'll talk about it in the [non-convex polygon collision detection section](#non_convex).

#### Thinking outside the box: polygon triangulation

As you can see, as simple as it can be, the Jordan curve theorem poses some problems that may be a bit out of our reach as of now, so let's try to find a less ideal but easier to understand solution.

Let's now limit ourselves to convex polygons, which (again) is the most common situation.

We can take inspiration from 3D graphics, where any solid shape (and thus the polygons that make those up) are decomposed to a bunch of triangles. Nothing stops us from doing the same and taking any polygon and decomposing it to a group of triangles, like follows:

![Decomposing a polygon into triangles](./images/collision_detection/polygon_triangulation.svg){width=40%}

This specific triangulation is called "fan triangulation" and it is chosen for its $\Theta(n)$ (where n is the number of vertices) execution time.

#### Bounding Boxes

Before making our poor CPU undertake big calculations, we may want to check if there is even a possibility of a collision, maybe with a simpler algorithm.

The great majority of the lifetime of our game objects is spent not colliding with anything, so if we can easily exclude a collision before starting complex algorithms, our game will just benefit from it.

We can take our complex polygon and give it a "bounding box", any point that is inside such box *has a possibility* of colliding with our polygon, but any point *outside* the bounding box *surely will not collide*.

![Example of a polygon with its bounding box](./images/collision_detection/bounding_box.svg){width=40%}

How do we calculate a bounding box? Simple, we just need 4 coordinates:

- The smallest x (which we'll call $x_{min}$)
- The smallest y ($y_{min}$)
- The biggest x ($x_{max}$)
- The biggest y ($y_{max}$)

The vertices of our bounding box will always be:

$$ A(x_{min}, y_{min})\ B(x_{max}, y_{min})\ C(x_{max},y_{max})\ D(x_{min}, y_{max})$$

::: tip :::
Thanks to how rectangles work, we can just use the points $A$ and $C$ to build a rectangle: since they contain all 4 coordinates, we can infer $B$ and $D$ from them.
:::::::::::

This is simple to achieve: we just need to loop over all the vertices and find our coordinates. The algorithm here below:

```{src='collisiondetection/bounding_box' caption='How to find the bounding box of a polygon'}
```

To check if the collision "may happen", we can just use a simple [Point vs Rectangle collision check](#point_rectangle).

#### Point/Polygon collision detection with triangles

Finally, after all the math and preparations, we can start working towards our collision detection algorithm.

::: pitfall :::
This algorithm works only with convex polygons that have no holes, also it probably is not the most efficient way to check for collisions between a point and a polygon.

This is more akin to an exercise in creativity and less about "notions": we found a simple solution to a complex problem. Even if it is not the most efficient, it may be "efficient enough".
:::::::::::::::

##### The "Polygon" class

Differently from previous classes and structures, the "polygon" class will need a little more work. This is because we are going to do more than just merely memorize vertices.

First of all we need an **ordered** list (or array) of vertices, which will be represented by points. Secondly, we need facilities to calculate list of triangles, as well as their areas.

::: pitfall :::
You may be tempted to memorize the "triangles" that are an output of the "fan triangulation", as well as their areas. This may be a good idea if well managed, but we will need to take care of "moving" those triangles and manage when the polygon gets deformed: in that case all the triangle areas will have to be recalculated.

Same goes for the bounding box, which will change in size when the polygon rotates or deforms. In this book we will try to keep the class as generic as possible (as well as simple), thus we will just recalculate everything every frame as needed.
:::::::::::::::

Thirdly, we need the constructor to do some math before we can use the polygon. Finally we need to integrate a "fanning" function.

Whew... That's a lot of work, but here's the code for the polygon class:

```{src='collisiondetection/polygon' caption='A (not so) simple polygon class'}
```

##### The algorithm

After all this preparation, we are finally ready for the algorithm, which will happen in two passes:

1. A "broad"-ish pass, where we compare the point to the polygon's bounding box
2. A "proper-narrow" pass, where we do a series of triangle vs point collision detections

Here's the code:

```{src='collisiondetection/polygon_point' caption='Polygon vs Point collision detection'}
```

##### Performance analysis

The algorithm seems fairly simple, but we may want to check its performance to see how efficient it is. In this analysis `n` will be the number of vertices, while `m` is the number of triangles.

The best case is that the point we're testing is outside the polygon's bounding box: this means that we calculate the bounding box (which is $\Theta(n)$) and we check the point against it (which is $\Theta(1)$), thus our best case (lower bound) is $\Omega(n)$.

The worst case is when the whole algorithm is performed to the end, which means the point is inside the bounding box, but outside the polygon: this means we calculate the bounding box ($\Theta(n)$), check against it ($\Theta(1)$), do the "fan triangulation" ($\Theta(n)$), check each triangle without finding any collision ($O(m)$) and get to the end. Out worst case (upper bound) is $O(n + m)$.

Considering the fact that the number of triangles `m` is tied to the number of vertices `n` by the formula (valid for simple convex polygons)

$$ m = n - 2 $$

We have an upper bound of $O(n+m) = O(n+n-2) = \sim O(n)$, this is because the constant gets "squashed by the linear behaviour" of `n`, and $2 \cdot n$ behaves asymptotically in the same way as $n$ when the dataset grows.

Even though we have a tight bound of $\Theta(n)$ in our entire algorithm (which means the amount of calculations goes up slowly with the addition of new vertices), we need to be mindful of the amount of calculation that is done, including some heavy operations like square roots.

### Circle/Polygon Collision

Now that we got one of the hardest topics out of the way, we can focus on other types of collision detection between arbitrary convex polygons: one of those is the "circle vs polygon" collision detection.

Let's see an example image first:

![Example image used for circle/polygon collision detection](./images/collision_detection/circle_polygon.svg){width=40%}

Here we can see four different cases of collision (or lack thereof) between a circle and a polygon (if you're particularly acute, you may have noticed we're missing a 5th case, but we'll talk about it shortly):

- **Case A:** The circle is completely inside the polygon;
- **Case B:** The circle is partially inside the polygon, with the center being **inside** the polygon;
- **Case C:** The circle is partially inside the polygon, with the center being **outside** the polygon;
- **Case D:** The circle is completely outside the polygon.

Case A and B can be solved together with a point/polygon check, where the point is the center of the circle, while case C can be solved by a line/circle check between the circle and all the edges of the polygon.

What about the "missing 5th case"? Here it is:

![An edge case of the circle/polygon check](./images/collision_detection/circle_polygon_2.svg){width=30%}

In this case the circle contains the polygon completely, with its center outside of the polygon area, so the check used in cases A and B wouldn't work and neither would the one used in case C.

This is a really rare edge-case, since usually the game does its checks so fast that you'd end up in case C long before this edge-case sees the light of day. In the event this happens, we just need to check if any of the vertices of the polygon is inside the circle.

Here's the full algorithm:

```{src='collisiondetection/circle_polygon' caption='Polygon vs Circle collision detection'}
```

### Line/Polygon Collision

The line vs polygon collision detection algorithm is not really different from the ones we have seen previously. Let's take a look at an image with all the cases we can think about:

![Example image used for line/polygon collision detection](./images/collision_detection/line_poly.svg){width=40%}

Here we can see 4 cases (this time for real):

- **Line $\overline{AB}$:** The segment is completely inside the polygon (including its ends);
- **Line $\overline{CD}$:** The segment is partially inside the polygon (one of its ends is inside the polygon);
- **Line $\overline{EF}$:** The segment crosses the polygon, but both its ends are outside the polygon;
- **Line $\overline{GH}$:** The segment is completely outside the polygon;

We can solve the cases involving the lines $\overline{AB}$ and $\overline{CD}$ by checking if either of the ends is inside the polygon, using a point/polygon collision check.

The case involving the line $\overline{EF}$ can be solved by a line/line collision check between the $\overline{EF}$ and all the edges of the polygon.

Let's take a look at the full algorithm:


```{src='collisiondetection/line_polygon' caption='Polygon vs Line collision detection'}
```

### Polygon/Polygon Collision

Here we are, the final frontier, polygon vs polygon collision detection. We went through a lot of pages of notions and reasoning to get here, now we have the tools to undertake one of the more complex collision detection methods.

Remember: we are checking if two **convex** polygons are colliding, let's see an example image first.

![Example image used for polygon/polygon collision detection](./images/collision_detection/polygon_polygon.svg){width=40%}

We can see 4 cases here, from the simplest to the hardest:

- The Square **D** is outside the polygon;
- The Pentagon **B** is completely inside the polygon
- The Octagon **E** is colliding with the heptagon **C** and a vertex of **C** is inside of **E**;
- The heptagon **C** is colliding with the hexagon **A**, but none of its vertices of **C** are inside of **A**;

We can easily solve the cases involving **A** and **E** with a "polygon vs line" collision detection, while the case involving **B** can be checked by doing a "polygon vs point" check.

Let's take a look at the algorithm:

```{src='collisiondetection/polygon_polygon' caption='Polygon vs Polygon collision detection'}
```

As you can see, the algorithm is quite short, but it builds on a lot of previous algorithms that we already studied, so there is a lot of "hidden complexity" behind these few rows of code.

::: tip :::
We can make the algorithm perform a bit better by adding a check between the (axis aligned) bounding boxes first: this will drastically reduce the amount of "line vs polygon" and "point vs polygon" checks, at the expense of a slightly heavier algorithm when a collision happens.
:::::::::::

### Non-convex polygons collision {#non_convex}

Let's go back to our previous example, using a non-convex polygon: we have an "inside point" and two points to test, one inside and one outside.

![How a non-convex polygon still makes everything harder](./images/collision_detection/jordan_curve_4.svg){width=50%}

The trick is counting the number of times our "segment between the points" hits the perimeter of the polygon:

![Counting how many times we hit the perimeter gives us the result](./images/collision_detection/jordan_curve_5.svg){width=50%}

If the number of "hits" is odd, we know the point tested is outside, if the number of "hits" is even, the point is inside the polygon.

The previous statement fails when we hit a vertex in our way: we can't really count it as a "double hit", because there's the possibility that we are hitting it while "entering" the polygon.

![Issues with vertices make everything even harder](./images/collision_detection/jordan_curve_6.svg){width=50%}

If we counted the vertex hit as a "double hit", we would end up having a point "inside the polygon" figuring as a "point outside the polygon".

The complications and edge cases are many and beyond the scope of this book, so we'll stop here and instead continue with the ways we discussed earlier.

#### Polygon triangulation: the return

We can extend the reasoning we made with simple convex polygons earlier to all simple polygons (so we can include non-convex ones too): any non-self-intersecting polygon without holes can be decomposed into triangles.

The only limitation we have is the method: the "fan triangulation" method works only with convex polygons and a very limited set of non-convex ones; so we need to find a different way of triangulating those polygons.

![Triangulating a non-convex polygon](./images/collision_detection/polygon_triangulation_2.svg){width=50%}

Triangulation methods include "ear clipping" and "monotone polygon triangulation", but their implementation is beyond the scope of this book.

::: tip :::
You can always take any type of polygon (even with holes) and decompose it into a bunch of convex polygons that can be fan-triangulated. Many graphical libraries represent polygons based on the fan-triangulation method.
:::::::::::

### Pixel-Perfect collision

Pixel perfect collision is the most precise type of collision detection, but it's also by far the slowest.

The usual way to perform collision detection is using **bitmasks** which are 1-bit per pixel representation of the sprites (white is usually considered a "1" while black is considered a "0").

![Two Bitmasks that will be used to explain pixel-perfect collision](./images/collision_detection/bitmasks.png){width=50%}

A logic "AND" operation is performed, pixel-by-pixel, on the bitmasks; with the sprite position taken in consideration, as soon as the first AND operation returns a "True" a collision occurred.

![Two Bitmasks colliding, the 'AND' operations returning true are highlighted in white](./images/collision_detection/bitmasks2.png){width=50%}

```{src='collisiondetection/pixel_perfect' caption='Example of a possibile implementation of pixel perfect collision detection'}
```

This algorithm has a time complexity of $O(n \cdot m)$ where $n$ is the total number of pixels of the first bitmask, while $m$ is the total number of pixels in the second bitmask.
