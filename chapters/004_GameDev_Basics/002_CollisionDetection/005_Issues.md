Common Issues with time-stepping Collision Detection
----------------------------------------------------

The methods we saw so far when checking for collisions are called "time-stepping techniques" due to the fact that each loop we "take a snapshot" of the situation and analyze it, this opens the door to a series of issues that may be annoying and we may find in our game development endeavors.

### The "Bullet Through Paper" problem {#bulletthroughpaper}

The "bullet through paper" is a common problem with collision detection, when an obstacle is really thin (our "paper"), and the object is really fast and small (the "bullet") it can happen that collision is not detected.

![Example of the "Bullet through paper" problem](./images/collision_detection/Bullet_Through_Paper.svg){width=40%}

The object is going so fast that it manages to go through the entirety of the obstacle in a single frame.

Possible solutions to this problems are various, some even going out of the realm of the so-called "time-stepping techniques" (like speculative contacts or ray casting) that can be very expensive from a computational standpoint.

Such solutions should therefore be enabled (or implemented) only for fast-moving objects and only if necessary, since resources and time are at a premium in most cases.

### Precision Issues

Sometimes it can happen that the position is reset incorrectly due to machine precision or wrong rounding, this can lead to the character that looks spazzy or just going through the floor at random times. The solution to these issues is making sure that the position and state are set correctly so that there are no useless state changes between frames.

Sometimes the "spazziness" of the character derives from the fact that collision reaction sets the character one pixel over the floor, triggering the "falling" state, the next frame the state would be changed to "idle" and then in the frame "n+2" the cycle would restart with collision reaction putting the character one pixel over the floor.
