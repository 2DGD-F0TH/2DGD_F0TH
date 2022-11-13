### Line/Rectangle Collision

Given the previous explanation about the Line/Line collision detection, it's quite easy to build a Line/Rectangle algorithm; distinguishing the cases where we want to account for a segment being completely inside of a rectangle or not.

```{src='collisiondetection/line_rectangle' caption='Implementation of the line/rectangle collision detection'}
```

This can prove useful to test for "line of sight" inside an AI algorithm.
