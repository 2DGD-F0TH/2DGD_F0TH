### Sprite sheets {#SpriteSheets}

Every time we create a sprite, we need some amount of memory to store its information, and to match the hardware constraints most of the time a sprite's image must be padded with unused pixels.

![Example sprite that gets padded to match hardware constraints](./images/resources/spritesheet_hardware_padding_sprite.png){width=30%}

Each sprite image that gets stored (and there could be potentially hundreds) wastes more and more memory, so we need a way to store sprites more efficiently. In the previous example we can see how a 21x21 sprite gets padded towards a 32x32 size, the sprite uses 52% more memory than it should!

Enter the humble Sprite Sheets.

We save our sprites (as well as animation frames) into a single drawing, called a "sprite sheet". By composing a sprite sheet with several smaller images of the same size, we just need to adapt our rendering to draw a portion of such sprite sheet on our screen. The sprite sheet is the only thing that will need to be adapted to match our hardware constraints, saving memory.

![Example spritesheet that gets padded to match hardware constraints](./images/resources/spritesheet_hardware_padding_sheet.png){width=40%}

In the previous example, the sprite sheet occupies only 1.5% more memory than it should. That's a great improvement

This way, instead of having a lot of references to sprites to draw, each one wasting its own memory, we just need the reference to the sprite sheet and a list of coordinates (rectangles, most probably) to draw.

Libraries like OpenGL support "sprite atlases" (or sprite batches), allowing for the graphics card to take care of drawing (after preparing the batch) while the CPU can use more of its cycles to take care of input, movement and collisions.

{{placeholder}}

<!-- TODO: Finish talking about sprite sheets -->
