### Point/Triangle Collision

A possible way to define if a point is inside a triangle, we can use a bit of geometry.

We can use *Heron's formula* to calculate the area of the original triangle, and compare it with the sum of the areas created by the 3 triangles made from 2 points of the original triangle and the point we are testing.

![Point/Triangle Collision Detection: division into sub-triangles](./images/collision_detection/point_triangle.svg){width=30%}

If the sum of the 3 areas (represented in different colors in the figure) equals to the original calculated area, then we know that the point is inside the triangle.

Let's see the code:

```{src='collisiondetection/point_triangle' caption='Point/Triangle Collision Detection'}
```

Let's see how we can change the algorithm to accomodate for some leeway, since the we may be requiring too much precision from our algorithms. We can do that by using epsilon values.

Our main test is that the sum of the area of the 3 triangles we create ($A_1,A_2,A_3$) is equal to the area of the original triangle ($A_0$), in math terms:

$$ A_1 + A_2 + A_3 = A_0$$

We can also rewrite such equation this way:

$$ A_1 + A_2 + A_3 - A_0 = 0$$

Due to possible precision issues we know that there are some values where the equation above is not true, so we choose a "low enough error" that we are willing to accept, for example $\epsilon = 0.0001$, and use this test instead:

$$ | A_1 + A_2 + A_3 - A_0 | < \epsilon$$

Which can be expanded (if you want) to

$$ -\epsilon < A_1 + A_2 + A_3 - A_0 < \epsilon $$

The code wouldn't change much, but for sake of clarity, here it is:

```{src='collisiondetection/point_triangle_epsilon' caption='Point/Triangle Collision Detection with epsilon'}
```
