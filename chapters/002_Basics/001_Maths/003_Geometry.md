Geometry
--------

Among all the maths we found so far (and the maths we will explain later), we cannot avoid talking a bit about geometry: in this book we will talk about the minimal amount of geometry necessary to understand the underlying concepts of what's coming up.

### Convex vs Concave polygons {#conc_conv}

A polygon is considered convex essentially when **any line** (not tangent to an edge or corner) drawn through the shape crosses the shape itself only twice (at its ends).

![Example of a convex shape](./images/maths/convex_shape.svg){width=35%}

Any shape where you can find at least one line that crosses the shape more than twice is considered "non-convex" (commonly referred as "concave").

![Example of a concave shape](./images/maths/concave_shape.svg){width=35%}

::: note :::
Not all non-convex shapes are technically called "concave" (they should be called "non-convex"), but for the sake of simplicity we'll use the term "non-convex" and "concave" interchangeably in this book.
:::::::::::::::

### Self-intersecting polygons

Contrary to what many think, polygons can self-intersect too, which can make calculations a lot harder.

![Example of a self-intersecting polygon](./images/maths/self_intersecting_poly.svg){width=40%}

For the sake of game development, we will usually talk about simple polygons which are polygons that don't self-intersect and have no holes in them. More strictly we will (for 99.9% of the time) talk about **convex simple polygons**.

### Straight Lines and their equations

One of the main topics we will encounter over and over in our game development adventure will be "straight lines". We will need to draw them, see if two straight lines collide, project stuff onto them, and much more. So it's important that we know them well.

Here's a straight line:

$$ ax + by + c = 0$$

That's not what you expected, right? What you've seen is the "general form" of a straight line's equation, because you can represent lines using equations (also circles, and other stuff). This is not a much-used form, though, probably the most used form is called the "slope-intercept form":

$$ y = mx + q$$

::: trivia :::
To transform a "general form" equation into the relative "slope-intercept from" just remember the following formulas:

$$ m = -\frac{a}{b}\ \ q = -\frac{c}{b}$$

This doesn't work well when $b=0$, which will be subject of the next "pitfall".
:::::::::::::

Where in this case $m$ is the *slope* of our straight line, and $q$ represents the so-called *y-intercept* (the value of $y$ when $x=0$). If $q=0$ the line goes through the origin of the Cartesian coordinate system, if $m=0$ the line is horizontal.

::: pitfall :::
"Vertical straight lines" is where the slope-intercept form fails, in fact vertical straight lines have an equation in the form of $x=k$, which would mean that $b=0$ which is problematic (see previous trivia).
:::::::::::::::

#### Getting the equation of a straight line, given two points

We all know that given two points we can strike one and only one line. How many times did you measure two points (maybe while doing some D.I.Y.) and stroke a line between them?

It will be useful in our adventure to be able to get the equation of a straight line starting from two points, so let's call our two points $P(x_1,y_1)$ and $Q(x_2,y_2)$, then the straight line that crosses both those points will have equation:

$$ \frac{y-y_1}{x-x_1} = \frac{y_2 - y_1}{x_2 - x_1}$$

This may seem really complicated, but with some small calculations we can reach a formula for our straight line in any form (generic or "slope-intercept").

::: pitfall :::
Again, this formula fails when we are dealing with "vertical lines", because the denominator at the right side of the equation will be zero. But in that case we'll already know the formula: it will be $x=x_1$ (which in turn will be equal to $x_2$)
:::::::::::::::

#### Getting the equation, given the slope and a point

If we have a point $P(x_p, y_p)$ and the slope $m$ (for instance if we need to find a line perpendicular to another line), in that case we can use the following formula:

$$ y - y_p = m(x - x_p)$$

::: pitfall :::
Guess what? This (again) doesn't allow us to create "vertical lines", because we need a slope value, which we don't have when it comes to vertical lines. You can see (non rigorously) a vertical line as a line with "infinite slope".
:::::::::::::::

### Projections {#projections}

In some situations (as you will see in the [SAT](#SAT)), we may need to get to project polygons onto a line, this usually involves projecting **points** to a line.

Given the formulas we've seen earlier, and doing some thinking, we can easily project a point onto any straight line. Let's see how to do it.

First of all, the line we will be projecting onto will have equation $y=mx+q$, just as in the slope-intercept formula.

We will assume that we have a point $P(x_p,y_p)$ that we want to project onto a line $r$ with equation $y=mx+q$, with $m \neq 0$ (thus excluding horizontal lines). We will call the projected point "P onto r" with the name $P_r(x_r,y_r)$.

![Projecting the point P onto the line r](./images/maths/projection.svg){width=40%}

First, we need to find the line that goes through $P$ and is perpendicular to $r$, this is really easy. To find a slope $m_1$ of a line perpendicular to another line with slope $m$ we use the formula

$$m_1 = \frac{1}{m}$$

::: pitfall :::
This is why we excluded the case $m=0$ (horizontal lines), if we didn't we would have the chance of having $m_1 = \frac{1}{0}$ which doesn't make sense.

In this case we can easily conclude that if $m=0$, the projection of the point $P$ onto the line $r$ has coordinates $(x_P, y)$ (with y taken from the line we're projecting onto).
:::::::::::::::

Now we have a point and a slope, so we can use one of the formulas we've already seen to find the line with that slope that crosses $P$:

$$y - y_p = m_1(x - x_p) \Leftrightarrow y - y_p = \frac{1}{m}(x - x_p)$$

To find $P_r$ we just need to find the point where the two lines collide, which is the solution to the equation system:

$$
\begin{cases}
y = mx + q\\
y - y_p = \frac{1}{m}(x - x_p)
\end{cases}
$$

Which finds solution in:

$$
\begin{cases}
x = \frac{x_p - my_p + mq}{1-m^2}\\
y = \frac{mx_p -m^2y_p + q}{1-m^2}
\end{cases}
$$

The coordinates $x$ and $y$ we just found are actually the coordinates $x_r$ and $y_r$ of our projected point $P_r$.

#### Projecting arbitrary lines on the axes

Similarly to what we've done with points, we can project arbitrary lines (or better, the ends of such lines) onto the axes. This will help us in doing some calculations later.

To project any line to the x-axis we can just "pass all the line's points through" the following function:

$$proj_x(x,y) = x$$

If we want to project such line on the y-axis, we can just use this other function:

$$proj_y(x,y) = y$$

We can see the graphical representation of projecting a line onto the axes below:

![Projecting a line onto the axes](./images/maths/projection_on_axes.svg){width=50%}
