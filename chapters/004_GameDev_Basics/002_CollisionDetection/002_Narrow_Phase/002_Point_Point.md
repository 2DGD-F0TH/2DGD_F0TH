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
