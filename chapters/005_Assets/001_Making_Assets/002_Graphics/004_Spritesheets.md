### Sprite sheets {#SpriteSheets}

Every time we create a sprite, we need some amount of memory to store its information, and to match the hardware constraints most of the time a sprite's image must be padded with unused pixels.

![Example sprite that gets padded to match hardware constraints](./images/resources/spritesheet_hardware_padding_sprite.png){width=30%}

Each sprite image that gets stored (and there could be potentially hundreds) wastes more and more memory, so we need a way to store sprites more efficiently. In the previous example we can see how a 21x21 sprite gets padded towards a 32x32 size, the sprite uses 52% more memory than it should!

Enter the humble Sprite Sheets.

We save our sprites (as well as animation frames) into a single drawing, called a "sprite sheet". By composing a sprite sheet with several smaller images of the same size, we just need to adapt our rendering to draw a portion of such sprite sheet on our screen. The sprite sheet is the only thing that will need to be adapted to match our hardware constraints, saving memory.

![Example spritesheet that gets padded to match hardware constraints](./images/resources/spritesheet_hardware_padding_sheet.png){width=40%}

In the previous example, the sprite sheet occupies only 1.5% more memory than it should. That's a great improvement.

This way, instead of having a lot of references to sprites to draw, each one wasting its own memory, we just need the reference to the sprite sheet and a list of coordinates (rectangles, most probably) to draw.

Libraries like OpenGL support "sprite atlases" (or sprite batches), allowing for the graphics card to take care of drawing (after preparing the batch) while the CPU can use more of its cycles to take care of input, movement and collisions.

Sprite sheets are also used in 3D games, usually under the name of "texture atlases": the objective of a texture atlas is reducing I/O operations and context switching by leveraging the [principle of locality](#locality_principle).

#### Sprite Sheet Gotchas

There are some issues when working with sprite sheets that we need to look for. They are definitely not deal-breakers but without care they may be the origin of hard-to-debug problems.

The first is texture compression: with some compression algorithms or certain compression factors, you may find out that each "sprite" contained in a spritesheet may end up influencing all the other sprites.

Let's take the following example:

![Two platforms in a spritesheet](./images/resources/spritesheet_no_compression.png){width=30%}

:::: longdesc ::::
Two futuristic-like platforms: on top there is a blue one, on the bottom there is a yellow one. The image is not compressed, so no artifacts are present.
::::::::::::::::::

Now let's compress the image heavily using JPEG and see what happens:

![Two platforms in a spritesheet after heavy compression](./images/resources/spritesheet_compression.png){width=30%}

:::: longdesc ::::
Two futuristic-like platforms: on top there is a blue one, on the bottom there is a yellow one. The image is heavily compressed and some yellow artifacts are visible on the blue platform.
::::::::::::::::::

As you can see, the compression artifacts bring some yellow onto the blue platform, which is not what we wanted.

The second issue can be thought as something similar to the first: when using mipmapping (pre-calculates sequences of images with progressively lower resolutions), usually in 3D games, the sprites may again influence each other.
