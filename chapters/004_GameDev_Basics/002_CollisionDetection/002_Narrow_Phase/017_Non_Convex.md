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
