Other Collision Detection Methods
---------------------------------

### Calculating the position of tiles

When you are using tiles to build a level, being able to use quad trees or brute force methods to limit the number of collision checks inside your game may be harder than other methods.

Using a bit of math is probably the easiest and most efficient method to find out which collisions happened.

Let's take an example level:

![Example tile-based level](images/collision_detection/Tile_Calc_Example_Level_1.png){width=40%}

If a game entity is falling, like in the following example:

![Tile-based example: falling](images/collision_detection/Tile_Calc_Falling.png){width=40%}

Using the simple AABB collision detection, we will need to check only if the two lowest points of the sprite have collided with any tile in the level.

First of all let's consider a level as a 2-dimensional array of tiles and all the tiles have the same size, it is evident that we have two game entities that work with different measures: the character moves pixel-by-pixel, the ground instead uses tiles. We need something to make a conversion.

Assuming `TILE_WIDTH` and `TILE_HEIGHT` as the sizes of the single tiles, we'll have the following function:

```{src='collisiondetection/tile_conversion' caption='Converting player coordinates into tile coordinates'}
```

To know which tiles we need to check for collision, we just have to check the two red points (see the previous image), use the conversion function and then do a simple AABB check on them.

```{src='collisiondetection/tile_collision' caption='Tile-based collision detection'}
```

Considering that this algorithm calculates its own colliding tiles, we can state that its complexity is `O(n)` with `n` equal to the number of possibly colliding tiles calculated.

If an object is bigger than a single tile, like the following example:

![Example tile-based level with a bigger object](images/collision_detection/Tile_Calc_Example_Level_2.png){width=40%}

We will need to calculate a series of intermediate points (using the `TILE_WIDTH` and `TILE_HEIGHT` measures) that will be used for the test

![Tile-based example with a bigger object: falling](images/collision_detection/Tile_Calc_Falling_2.png){width=40%}

And using the same method the colliding tiles can be found without much more calculations than the previous algorithm, actually we can use exactly the same algorithm with a different list of points to test.

### The "Tile + Offset" Method

This is a really good trick that works well for games that are heavily based on grids: the player can move only in the four cardinal directions and movement is tile-based.

By "tile-based movement" I mean that if you press any direction for even the smallest amount of time (even a single frame), the player will move in that direction by a tile (however big it may be).

The idea behind this kind of collision detection is very simple: some tiles are marked as walls. When the player wants to move in a certain direction, the game will check the tile in the chosen direction, if it's a wall the movement will be blocked, if it's passable the game will tween (usually using an offset parameter) the player travelling between tiles.

{{placeholder}}

<!-- TODO: Useful for games like pacman, check the direction where you are going using the offset, if the next cell is a wall, react -->
