Separating Axis Theorem {#SAT}
-----------------------

We have taken an in-depth look at a series of specialized algorithms, but there is a more generic theorem that allows us to determine if two convex polygons are colliding: The *Separating Axis Theroem* or SAT. This theorem states:

> If two convex objects are not penetrating, there exists an axis for which the projection of the objects will not overlap.

This is connected to a simpler "human" explanation, which is:

> If two convex polygons are not colliding, then you can draw a straight line between them.

![Example of how you can draw a line between two convex non-colliding polygons](./images/collision_detection/SAT1.svg){width=35%}

Before delving further into the matter, let's see what we need to know:

- [The difference between a Convex and a Concave Polygon](#conc_conv)
- [What a Projection is](#projections)
- [Some Vector Maths](#vectors)

### Why only convex polygons?

To explain this, we'll use the "human explanation": if one of the shapes is concave, there is a possibility that the polygons are not colliding, but we cannot draw a straight line between them.

![Why the SAT doesn't work with concave polygons](./images/collision_detection/SAT2.svg){width=35%}

Thus our algorithm would return a collision where there is none.

::: tip :::
This problem can be solved by "decomposing" the concave polygons in two or more convex polygons, but for the sake of semplicity we'll assume all polygons we are checking for collisions are convex.
:::::::::::

Now let's check the more "technical explanation".

### How it works

Let's read the definition of the separating axis theorem again and break it down:

> If two convex objects are not penetrating, there exists an axis for which the projection of the objects will not overlap.

The first part defines the condition: in case two objects are not colliding, then what follows is true.

For what we were concerned so far, axes were "aligned to the screen boundaries", but axes can actually have different orientations and we can project shapes onto them.

The condition in our definition is represented as follows:

![How the SAT algorithm works](./images/collision_detection/SAT3.svg){width=50%}

As we can see, we have found an axis (which in this case is slanted) where the projection of the two shapes don't overlap. The presence of this axis where the projections don't overlap is guaranteed by the fact that the two polygons don't collide.

::: trivia :::
We can now easily see why the "human explanation" is (for our own purposes) equivalent to the "technical" one: we just need to take a single point inside the "gap" between the two projections and strike a line perpendicular to our axis.

That's our "separating axis".
::::::::::::::

#### Finding the axes to analyze

Now we only have a problem: we definitely can't spend an infinite amount of time trying all possible combinations in the hope of finding an axis where the projections don't overlap.

The fact is: we don't need to try them all. Actually we need to try just a few, as many as the sides of the polygons involved.

The axes we need to check are actually the axes parallel to the "normal of the polygon's edges". In layman's terms: the axes we need to check are parallel to lines which are perpendicular to the edges of our polygons.

Let's take it step by step, first we find the "normals", which are just unit vectors perpendicular to the edges of our polygons.

![Finding the axes for the SAT (1/2)](./images/collision_detection/SAT_Axes_1.svg){width=50%}

Now we just have to strike axes parallel to those normals, and those are the axes we will need to check against.

![Finding the axes for the SAT (2/2)](./images/collision_detection/SAT_Axes_2.svg){width=50%}

In the previous pictures, I chose axes around the two polygons, for the sake of clarity.

::: pitfall :::
Do not think that the axes we found are 5: there actually are 10. This is due to the fact that the figures I chose (for the sake of cleanliness) are a rectangle and an hexagon, which have edges that are parallel in groups of two.
:::::::::::::::

#### Projecting the shapes into the axes and exiting the algorithm

Now, for each axis we found, we need to perform a projection of the two polygons onto such axis.

![Projecting the polygons onto the axes](./images/collision_detection/SAT_Projection.svg){width=50%}

Now we consider each axis on its own and see if the projections overlap.

As soon as we find an axis where the two projections don't touch (overlap), we know that the two polygons are not colliding. Thus we exit the algorithm.

If all the axes we scan have overlapping projections, we can say that the polygons we're analyzing are colliding.

In the example, we can find two axes that have non-overlapping projections, thus the worst case is that the algorithm misses both of them 3 times in a row and exits at the fourth iteration.

::: trivia :::
If you use Axis-Aligned rectangles as your "polygons", you will notice how the Separating Axis Theorem will degenerate into something very similar to a simple AABB collision detection.

The only difference is that we're checking a condition where the rectangles **don't** collide.
::::::::::::::

Due to its nature, this algorithm has higher efficiency when there are few collisions, since it exits as soon as we find a separating axis (a gap in the projections).

#### From arbitrary axes to "x and y"

The only thing that remains is how to switch from an "arbitrary axis" to our usual "x and y" axes. Here projections will help us again: we can simply project our projections.

![Projecting our projections onto the x and y axes](./images/collision_detection/SAT_Projection_2.svg){width=50%}

If we look closely, we're just projecting polygons onto a bunch of axes so that they get "flattened to lines", then we're projecting such lines onto the x and y axes to see if there those lines are touching or not.

Ray Casting
-----------

Sometimes it can necessary to use unusual techniques to detect collisions: ray casting is one of them. If well used (and with some "illusion magic"), ray casting can be a nice way to solve the "bullet through paper" problem.

### What is Ray Casting?

Mostly used in 3D, ray casting is a technique where you cast an imaginary ray (usually of light) until it hits something, but its uses can go beyond that.

Let's take for example shooting a simple bullet: this can give some issues when the "bullet" is small and fast, as explained earlier in [The bullet through paper problem](#bulletthroughpaper).

First of all, let's put up some (arbitrary) constraints that will help us making the computation easier and better performing without giving away our tricks too easily.

Our bullet will shoot from the barrel of our gun (duh!), but we also define a point where the bullet will despawn: this will limit our ray length and make our algorithm perform better. We can still give an excuse such as "bullets are affected by gravity" (which actually is true), and maybe use it as a difficulty management technique (stopping people from sniping the enemy can make the game harder and force the player to play the game the way we, the developers, want).

![How Ray Casting Works: Gun (1/2)](./images/collision_detection/ray_casting_1.svg){width=50%}

Attached to our gun, is an invisible line (our ray), that will follow every movement of the gun itself

![How Ray Casting Works: Gun (2/2)](./images/collision_detection/ray_casting_2.svg){width=50%}

When we want to shoot the gun, instead of using the previously stated "time-stepping techniques", we perform a line-to-rectangle (or line-to-circle, or whatever we find best) collision detection, at the same time we play a really fast animation of the bullet shooting along the casted ray. If the cast ray hits an enemy, they'll die (or get destroyed).

:::: tip ::::
If you find that the bullet animation won't align well with the enemy dying, the animation may not be fast enough. Some games even give up showing the bullet at all, and instead show a white line for a split second, that fades away. The effect works really well!
::::

{{placeholder}}

<!-- TODO: Talk about using ray casting to fix the tunneling problem (code?) -->

### Other uses for ray casting: Pseudo-3D environments

{{placeholder}}

<!-- TODO: Talk about how ray casting can be used to create pseudo-3d (DOOM-style) environments -->
