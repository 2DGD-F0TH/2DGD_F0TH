Top-view RPG-Like Games
-----------------------

### Managing height

When it comes to Top-view RPG games, height is a way to give your game a lot more visual appeal. Let's see how we can manage height in our games.

#### Faking it

The simplest way to manage height in a Top-down RPG game is to not do so at all. If you have a good tileset the player may not even realize it.

Let's take the following simple example:

![A simple example of fake height in RPG [^dungeon]](./images/developing_mechanics/fake_height_1.png){width=50%}

This can be seen as a "flattened" screen, where there are few collisions, while the tileset will "sell the effect".

![How few collisions may "sell" the effect of height [^dungeon]](./images/developing_mechanics/fake_height_2.png){width=50%}

In the previous image, the red sections are the tiles where collision is present: the stairs have nothing special, they are treated as any other "flat ground", but the texture sells the effect of stairs.

We can go as complex as we want:

![A more complex example of fake height [^dungeon]](./images/developing_mechanics/fake_height_3.png){width=50%}

And still have a somewhat simple and flat path, from bottom to top.

![Even with complex tilemaps, the texture sells the height effect [^dungeon]](./images/developing_mechanics/fake_height_4.png){width=50%}

If you're good at map building, this simple way of doing things can get you far enough to make a convincing effect and give variety to your environments without having to delve into complex algorithms that involve swapping layers or something similar.

[^dungeon]: Dungeon Tileset, listed as Public Domain at [https://opengameart.org/content/dungeon-tileset](https://opengameart.org/content/dungeon-tileset)

#### Mananging height for real

{{placeholder}}

<!-- TODO: How to manage the different "heights" in the game -->
