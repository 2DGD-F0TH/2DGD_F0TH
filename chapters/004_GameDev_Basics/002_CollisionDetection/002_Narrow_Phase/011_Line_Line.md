### Line/Line Collision

Line/Line collision is quite simple to implement once you know the inner workings of geometry, but first we need to explain the thought behind this algorithm, so... **math warning!!**

Let's look at the following image:

![Example image for line/line collision](./images/collision_detection/line_line.png){width=60%}

A generic point $P_a$ of line A can be represented with the following formula:

$$ P_a = P_1 + u_a \cdot (P_2 - P_1) $$

which translates into the coordinate-based equations:

$$
\begin{cases}
x_a = x_1 + u_a \cdot (x_2 - x_1)\\
y_a = y_1 + u_a \cdot (x_2 - x_1)
\end{cases}
$$

This makes us understand that any point of line A can be represented by its starting point $P_1$, plus a certain fraction (represented by $u_a$) of the vector represented by $P_2 - P_1$.

This also means that $0 \leq u_a \leq 1$, else the point won't be on the segment.

In the same way, a generic point $P_b$ of line B can be represented with:

$$ P_b = P_3 + u_b \cdot (P_4 - P_3) $$

which becomes:

$$
\begin{cases}
x_b = x_3 + u_b \cdot (x_4 - x_3)\\
y_b = y_3 + u_b \cdot (x_4 - x_3)
\end{cases}
$$

The two lines will collide when $P_a = P_b$, so we get the following equations:

$$
\begin{cases}
x_1 + u_a \cdot (x_2 - x_1) = x_3 + u_b \cdot (x_4 - x_3)\\
y_1 + u_a \cdot (y_2 - y_1) = y_3 + u_b \cdot (y_4 - y_3)
\end{cases}
$$

That need to be solved in the $u_a$ and $u_b$ variables.

The result is:

$$
\begin{cases}
u_a = \frac{(x_4 - x_3) \cdot (y_1 - y_3) - (y_4 - y_3) \cdot (x_1 - x_3)}{(y_4 - y_3) \cdot (x_2 - x_1) - (x_4 - x_3) \cdot (y_2 - y_1)}\\
u_b = \frac{(x_2 - x_1) \cdot (y_1 - y_3) - (y_2 - y_1) \cdot (x_1 - x_3)}{(y_4 - y_3) \cdot (x_2 - x_1) - (x_4 - x_3) \cdot (y_2 - y_1)}
\end{cases}
$$

Substituting either of the results in the corresponding equation for the line will give us the intersection point (which may be useful for some particle effects).

Now some notes on our solution:

- If the denominator for the equations for $u_a$ and $u_b$ equals to zero, the two lines are parallel
- If both the numerator and denominator for $u_a$ and $u_b$ are equal to zero, the two lines are coincident
- If both $0 \leq u_a \leq 1$ and $0 \leq u_b \leq 1$ then the two segments collide.

Now we can translate all this math into code:

```{src='collisiondetection/line_line' caption='Implementation of the line/line collision detection'}
```

This collision detection algorithm can be useful for line-based puzzle games, line the untangle puzzle.
