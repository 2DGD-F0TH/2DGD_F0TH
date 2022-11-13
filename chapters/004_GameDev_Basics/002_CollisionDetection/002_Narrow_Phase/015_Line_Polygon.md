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
