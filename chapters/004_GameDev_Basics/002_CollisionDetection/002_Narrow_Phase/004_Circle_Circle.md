### Collision Between Two Circles {#circle_circle}

Let's add another circle into the mix now, and think in more or less the same way as before:

![Reference image for Circle-Circle collision detection](./images/collision_detection/circle_circle.svg){width=50%}

We can see the distance between the center of the circles as expressed with the following formula:

$$ d = r_1 + x + r_2 $$

Where $r_1$ and $r_2$ are the radii, and $x$ is defined as follows:

$$ x = d - (r_1 + r_2) $$

As before, our $x$ can be negative, which means that the circles are colliding if $x \leq 0$, which means:

$$ x \leq 0 \Leftrightarrow d - (r_1 + r_2) \leq 0 \Leftrightarrow d \leq r_1 + r_2$$

We can express the concept in words again:

> Two circles are colliding when the distance between their centers is less or equal the sum of their radii

In pseudo code this would be:

```{src='collisiondetection/circle_circle' caption='Circle to Circle Collision Detection'}
```

The shorter version would be:

```{src='collisiondetection/circle_circle_lazy' caption='Shorter Version of a Circle to Circle Collision Detection'}
```

Again, this algorithm performs a number of operations that is constant, so it runs in O(1).
