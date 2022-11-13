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
