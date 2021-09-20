{{pagebreak}}

Collision Detection and Reaction
=================================

:::::: {.epigraph author="William Whewell"}
Every detection of what is false directs us towards what is true: every trial exhausts some tempting form of error.
::::::

When it comes to collision management, there are two main phases:

- **Collision Detection:** you find out which game objects collided with each other;
- **Collision Reaction:** you handle the physics behind the collision detected, making the game objects react to such collision.

Collisions don't only happen between game objects (two fighters hitting each other), but also between a character and the world (or they would end up just going through the ground).

In this section we'll talk about some ways you can detect and react to collisions.

Why Collision Detection is done in multiple passes
--------------------------------------------------

Collision detection algorithms can be quite costly, even more when you are using a [brute force approach](#brute_force), but it's possible to have a more precise collision detection at a lower cost by combining different collision detection algorithms.

The most common way to apply a multi-pass collision detection is by dividing the process in a "broad" and a "fine" pass.

The broad pass can use a very simple algorithm to check for the possibility of a collision, the algorithms used are usually computationally cheap, such as building quad trees.

When the simpler algorithm detects the possibility of a collision, a more precise algorithm is used to check if a collision really happened, usually such finer algorithms are computationally expensive and will benefit from the first "broad pass" filter, thus avoiding useless heavy calculations.

::: note :::
In this chapter we'll see the easier narrow-pass detection first, followed by the more complex broad-pass algorithms, but remember that a good collision detection system does a "broad-pass" first, before delving into the "narrow-pass".
::::::::::::

