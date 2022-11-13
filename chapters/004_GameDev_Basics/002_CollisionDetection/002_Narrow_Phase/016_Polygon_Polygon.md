### Polygon/Polygon Collision

Here we are, the final frontier, polygon vs polygon collision detection. We went through a lot of pages of notions and reasoning to get here, now we have the tools to undertake one of the more complex collision detection methods.

Remember: we are checking if two **convex** polygons are colliding, let's see an example image first.

![Example image used for polygon/polygon collision detection](./images/collision_detection/polygon_polygon.svg){width=40%}

We can see 4 cases here, from the simplest to the hardest:

- The Square **D** is outside the polygon;
- The Pentagon **B** is completely inside the polygon
- The Octagon **E** is colliding with the heptagon **C** and a vertex of **C** is inside of **E**;
- The heptagon **C** is colliding with the hexagon **A**, but none of the vertices of **C** are inside of **A**;

We can easily solve the cases involving **A** and **E** with a "polygon vs line" collision detection, while the case involving **B** can be checked by doing a "polygon vs point" check.

Let's take a look at the algorithm:

```{src='collisiondetection/polygon_polygon' caption='Polygon vs Polygon collision detection'}
```

As you can see, the algorithm is quite short, but it builds on a lot of previous algorithms that we already studied, so there is a lot of "hidden complexity" behind these few rows of code.

::: tip :::
We can make the algorithm perform a bit better by adding a check between the (axis aligned) bounding boxes first: this will drastically reduce the amount of "line vs polygon" and "point vs polygon" checks, at the expense of a slightly heavier algorithm when a collision happens.
:::::::::::
