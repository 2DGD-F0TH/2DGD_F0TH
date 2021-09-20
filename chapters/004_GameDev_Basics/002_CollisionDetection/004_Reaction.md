Collision Reaction/Correction
------------------------------

When you are sure, via any algorithm, that a collision has occurred, you now have to decide how to react to such collision. You may want to destroy the player or the target, or you may want to correct the behaviour, thus avoiding items getting inside walls.

### HitBoxes vs HurtBoxes

First of all, we need to explain the difference between a "HurtBox" and a "HitBox".

Such difference can be more or less important, depending on the game that is coded, and sometimes the two concepts can be confused.

A **HitBox** is a shape (usually a rectangle, see [Collision Between Two Axis-Aligned Rectangles (AABB)]) that is used to identify where a certain entity can *hit* another entity. For the player a "hitbox" could encase their sword while attacking.

A **HurtBox** is instead a shape that is used to identify where a certain entity can *get hurt* by another entity. For the player a "hurtbox" could be their body.

![Example of a hitbox (red) and a hurtbox (blue)](./images/collision_detection/hithurt.png){width=40%}

### Collision Reaction Methods

It has happened: a collision occurred and now the two objects are overlapping.

How do we react to this event in a convincing (not necessarily "realistic") and efficient manner? There are a lot of methods to react to collisions and below we will show some of the most used, along with some interesting ones.

We will use the following image as reference for each collision reaction:

![Images used as a reference for collision reaction](./images/collision_detection/reaction_reference.png){width=40%}

We will study each case separately, and each case will be a piece of this reference image.

#### The Direction + Velocity Method

This is the simplest method, computationally speaking: as soon as the objects gets inside of a wall, you push it back according to the direction its velocity has or just the direction of the character itself.

##### How it works

This works when you treat the `x` and `y` axis separately, updating one, checking the collisions that come up from it, update the other axis and check for new collisions.

```{src='collisiondetection/direction_velocity' caption='Code for the direction + velocity collision reaction'}
```

##### Analysis

Let's see how this method reacts in each situation.

When we are trying to fall on the ground, this method works as follows:

![How the direction + velocity method reacts to collisions on a horizontal plane](./images/collision_detection/direction_velocity_reference.png){width=80%}

1. We divide the movement vector in its `x` and `y` components.
2. We move along the `x` axis and check for collisions, in this case there are none (the ghost represents our previous position.
3. We move along the `y` axis, after checking for collisions we find that we are colliding on the ground (the ghost represents our next position).
4. We react to the collision by moving the sprite on top of the ground.

##### Quirks and issues

This method can be used only with completely solid platforms. If you want to make use of platforms that you can cross one-way, since you may get teleported around when your velocity changes direction.

![How velocity changing direction can teleport you](./images/collision_detection/velocity_teleport.png){width=80%}

In the previous example we try to jump on a platform by going through it, but our jump quite doesn't make it. Since velocity has changed direction, we end up being teleported over the platform, which is considered a glitch.

#### Shallow-axis based reaction method

This method works in a similar fashion to the direction and velocity method, but prioritizes reactions on the axis that shows the shallowest overlap.

This requires measuring how much the objects overlap on each axis, which can be a little more involved, but not really expensive.

![Example of shallow-axis based reaction](./images/collision_detection/shallow_axis.png){width=90%}

In the previous picture, we can see how the algorithm chooses to solve the collision on the $y$ axis first and only on the x axis after; but since solving the $y$ axis solves the collision, no reaction is performed on the $x$ axis.

{{placeholder}}

<!-- TODO: Similar to direction + velocity, but reacts only on the most shallow direction -->

#### The "Snapshot" Method

This method is a bit more involved, but allows for a finer control over how you go through or collide with certain obstacles.

The secret to this method is taking a snapshot of the object's position before its update phase and do a series of comparisons with the position after the update.

```{src='collisiondetection/snapshot_reaction' caption='Example of the "snapshot" collision reaction method'}
```

This method solves the problem given by platforms that can be crossed one-way.

<!-- TODO: Snapshot before and after updating, react accordingly, allows for more advanced stuff -->

#### The "Tile + Offset" Method

{{placeholder}}

<!-- TODO: Useful for games like pacman, check the direction where you are going using the offset, if the next cell is a wall, react -->
