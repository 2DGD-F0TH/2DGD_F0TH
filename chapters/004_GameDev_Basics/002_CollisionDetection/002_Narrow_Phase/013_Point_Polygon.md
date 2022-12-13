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

#### Point/Polygon collision detection using triangulation

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
