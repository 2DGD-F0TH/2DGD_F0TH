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

Although slightly more heavy, computation-wise, this algorithm still runs in O(1).
