### Pixel-Perfect collision

Pixel perfect collision is the most precise type of collision detection, but it's also by far the slowest.

The usual way to perform collision detection is using **bitmasks** which are 1-bit per pixel representation of the sprites (white is usually considered a "1" while black is considered a "0").

![Two Bitmasks that will be used to explain pixel-perfect collision](./images/collision_detection/bitmasks.png){width=50%}

A logic "AND" operation is performed, pixel-by-pixel, on the bitmasks; with the sprite position taken in consideration, as soon as the first AND operation returns a "True" a collision occurred.

![Two Bitmasks colliding, the 'AND' operations returning true are highlighted in white](./images/collision_detection/bitmasks2.png){width=50%}

```{src='collisiondetection/pixel_perfect' caption='Example of a possibile implementation of pixel perfect collision detection'}
```

This algorithm has a time complexity of $O(n \cdot m)$ where $n$ is the total number of pixels of the first bitmask, while $m$ is the total number of pixels in the second bitmask.
