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
