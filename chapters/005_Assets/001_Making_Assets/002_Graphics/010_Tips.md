### Tips and Tricks

This section contains various tips and tricks used by artists to create certain effects inside video games, demonstrating how sometimes something really simple can have a great effect on the game experience.

#### Tiles

##### Getting started

When you are starting to make a new tileset, it's a good idea to begin with a base sized 5x5 tiles or more (so if your single tile is 32x32 pixels, the image will be at least 160x160 pixels): this can give you variations on the tileset that won't make the "tiling" (repetitions) so easy to catch when the map is made.

![Example of tile "alternatives"](./images/resources/tile_alternatives.png){width=60%}

If you think something does not fit in a tileset, try to think what the surrounding area would look like: darker grass could be overgrown, a darker spot in the water could signify deeper water, shadows and light help too.

When it comes to "corner tiles", using the same tile, rotated in 90 degree steps is a great basis to build upon, after that you can edit the tiles accordingly.

This "rotation trick" can be used for most of the tiles you create, let's take for instance the following diagram, representing some tiles:

![Tile "Rotation Trick" (1/3)](./images/resources/tile_rotation_diagram_1.svg){width=60%}

We can take the tiles and rotate them, to get something like the following:

![Tile "Rotation Trick" (2/3)](./images/resources/tile_rotation_diagram_2.svg){width=60%}

If we continue with copying, rotating and pasting, we can obtain a great basis for our tileset:

![Tile "Rotation Trick" (3/3)](./images/resources/tile_rotation_diagram_3.svg){width=60%}

After that we can edit and make it so tiles are seamless, while putting the minimum amount of necessary effort to create something convincing.

##### Creating "Inside rooms" tilesets

In many cases, when dealing with tile-based games, we need to create a tileset that is good to represent "inside" environments, like a basement, a cave or the inside of a building. A simple way to reach that goal is creating a set of black and transparent tiles that can be overlaid on another tileset, like the following:

![Example of black and transparent tileset used in "inside rooms"](./images/resources/inside_tileset.png){width=30%}

Such tiles can then be overlaid onto something like the following:

![Example of incomplete "inside room"](./images/resources/inside_example_1.png){width=40%}

And we obtain the following result:

![Example of "inside room" with the black/transparent overlay](./images/resources/inside_example_2.png){width=40%}

{{placeholder}}

<!-- TODO: Finish talking about "inside room" tilesets -->

#### Sprites and icons

##### Shape first

When you're making some kind of sprite or icon, you should always get the basic shape of the object down first, then you can give the object more depth and detail with colors. This will help you understanding the space occupied by your object.

![How color can completely change an object](./images/resources/color_change_object.png){width=60%}

It's a puppy! It's a jester! It's a... hastily drawn flower vase?

This is the power of color, you can change the entire nature of an object by changing how it's colored, but having the basic shape of the object down first will help you a long way.
