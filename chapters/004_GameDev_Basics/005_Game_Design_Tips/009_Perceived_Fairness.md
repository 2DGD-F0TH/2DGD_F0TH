Perceived Fairness
------------------

### You don't need precise collision detection

Here we go, spouting bold claims again. Here's a bolder one: you don't **want** precise collision detection. It's slower, harder to implement and most of all, the player may get annoyed at it.

In the heat of a gaming session, with all the action going on, the player may become a bit "blind" to small things: this means that they will perceive as arbitrary anything that is not "evident".

Let's take this arrow vs "generic jumping man" example:

![A pixel perfect detection would trigger in this case](./images/game_design/collision_1.png){width=40%}

With still images, things are obvious: that's a collision, it's on the tip of the character's toe, but it's a hit. It may be 1 or 2 pixels (it's actually 3), but it is still a hit.

Now let's imagine the image moving: the arrow darting from left to right while the character is ascending, it looks like a near-hit, the player will appreciate the adrenaline rush of a near-death. If we go and declare that as a "hit", the player won't understand why, they will say "that's unfair, it just missed me". Our collision detection was **too precise**.

![A smaller hitbox may save the player some frustration](./images/game_design/collision_2.png){width=40%}

Sometimes we just need a hitbox that is small enough to avoid these "looks-like-a-miss" incidents: the instances where the collision detection triggers are evident and the player will appreciate the sensation given by the near-deaths.

Obviously you need to strike a balance, if the hitbox is so small that evident hits are counted as misses (with few exceptions, like [bullet hells](#bullethell)) it will break the immersion.

### Immediate dangers should be well visible

One of the biggest frustrations a player can encounter is definitely being damaged (or killed) by a hidden object.

To avoid these "unfair shots", we should draw immediate dangers "late" in the drawing phase of the game loop, but there is an issue: we don't want to break the immersion.

Let's take the following example: a bomb gets spawned just behind a chest by an enemy, and our character is dangerously close to it.

![A bomb spawned behind a chest, drawn in a realistic order](./images/game_design/dangers_1.svg){width=30%}

If the bomb is "behind" the chest, we can't suddenly make it pop "in front of" the chest, that would definitely ruin the immersion, also it may end up messing with the forced perspective that some 2D games use.

![Moving the bomb in front of the chest may ruin immersion](./images/game_design/dangers_2.svg){width=30%}

As you can see, even though the bomb has a shadow, it looks like the bomb and the shadow are "floating mid-air", thus ruining immersion.

Different games implement different solutions to the problem, some prefer highlighting the danger with an outline drawn over everything, something like the following:

![Highlighting the hidden part of a danger can be useful](./images/game_design/dangers_3.svg){width=30%}

Other instead prefer making the "foreground objects" semi-transparent, so that the player can see what lies behind.

![Making objects transparent is a solution too](./images/game_design/dangers_4.svg){width=30%}

This solution is usually applied when the player themselves are behind the obstacle, giving a more "interactive" and "less confusing" feel to the entire game.
