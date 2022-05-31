Graphics {#GraphicsResources}
---------

### Some computer graphics basics

Before we start doing anything, we need to know some basics about graphics. In this section we will introduce some terms like "bit depth" (or "color depth"), "filtering", "sprite sheets", "virtual resolution" and others.

We will see a lot of stuff, from a bird-eye view of how graphics are stored in video memory to normal maps and more.

#### The "color wheel"

We can't start talking about colors without mentioning primary and secondary colors, and have a "color wheel" accompany the explanation.

![The "color wheel" for screens](./images/resources/color_wheel.svg){width=40%}

In art classes, you've probably learned that primary colors were Magenta, Yellow and Cyan. On screens it's a bit different (this is due to how light works, compared to paint): on screens we have Red, Green and Blue as primary colors (thus making up the "RGB system").

In fact if you looked really close on old CRT screens, you would have probably seen a lot of red-green-blue pixels that together, seen from afar, would have made up the millions of colors that you've seen for a long time.

By mixing up the 3 primary colors, you would have obtained the secondary colors for light, which are... Magenta, Yellow and Cyan!

:::: trivia ::::
This is due to how light and paint work differently. It is said that "lights are additive" and "paints are subtractive": if you mix Magenta, Yellow and Cyan paints using minuscule dots (like in printing), you will see that Red, Green and Blue are formed. This is obviously an idealization, the colors you'd actually obtain by mixing paints by hand would be closer to Orange, Purple and Dark Green. In short, this is how inkjet printing works (also a Black component is added, thus forming the CMYK model).
::::::::::::::::

#### Color representations

In computers, there are two main representations for colors: the RGB and the HSV representations.

##### RGB Representation

We can memorize our colors by addressing their primary color components: Red, Green and Blue. This is usually done with 3 pairs of bytes, one for each "color channel" (component).

![An example of an RGB picker](./images/resources/RGB.svg){width=40%}

This means that you can represent color with a 3-tuple: $(RRR, GGG, BBB)$, where each channel can take a value from $0$ to $255$ (or $00$ to $FF$ in hexadecimal, in that case the color is usually prepended by a `#` symbol) if we consider the more commonly used "16 million colors" scheme.

- Pure red is represented with the 3-tuple $(255, 0, 0)$ (or `#FF0000` in hex)
- Pure green with $(0, 255, 0)$ (or `#00FF00`)
- Pure blue with $(0, 0, 255)$ (or `#0000FF`).

Black is the absence of any color component, which means it's represented with the $(0, 0, 0)$ 3-tuple (of `#000000`), while White is represented with $(255, 255, 255)$ (`#FFFFFF`).

##### RGBA Representation

Sometimes we need to represent transparency, in that case we need an extra pair of bytes to do so. In this case we talk about the "RGBA syste" (Red-Green-Blue-Alpha).

Colors are represented by a 4-tuple: $(RRR, GGG, BBB, AAA)$, where each channel can take a value from $0$ to $255$ (or $00$ to $FF$ in hexadecimal).

##### HSV Representation

Another way to represent colors is using the HSV system (Hue, Saturation, Value). Sometimes this sytem is also called HSB (Hue, Saturation, Brightness).

![A simplified "slice" of an HSV representation](./images/resources/HSV.svg){width=40%}

Using the reference image above, the Hue is selected by choosing an angle on the circle: we find pure red at $0\degree$, then proceeding counterclockwise we have pure green at $120\degree$, blue at $240\degree$ and then we go back to red at $360\degree$ (which is back at $0\degree$). This means that it's represented as a value between $0$ and $359$.

Saturation can be chosen by getting farther or closer to the center, with the minimum saturation being in the center and maximum at the outside. Saturation can be described as the "colorfulness of something compared to its brightness", which would mean that the color feels "less white-y" the higher the saturation. It is represented with a number between $0$ and $100$.

![More slices of the HSV representation show how value changes](./images/resources/HSV_Cylinder.png){width=30%}

Value (or brightness) is a bit different: the HSV representation is actually a cylinder (thus the use of "slice" in the figure description): you can imagine a stack of slices that get darker and darker the closer we get to the bottom of the cylinder. This too is represented with a number between $0$ and $100$.

:::: note ::::
Following this definition we can define pink as a "less saturated" red, as long as the brightness stays high (something like $(0, 50, 100)$). If the brightness is low (towards the lower half) we would obtain a dull brick red (for instance $(0, 50, 50)$).
::::::::::::::

#### Primary, Secondary and Tertiary colors

Primary colors are the basis of our color system and their definition is complex and outside the scope of this book. Let's just assume that they are the colors that we can mix together to "make other colors".

On screens, the primary colors are Red, Green and Blue; while in classic painting green is usually substituted by Yellow. If we wanted to be more precise, painting uses Magenta, Cyan and Yellow.

Secondary colors are made by mixing, in equal parts, two primary colors: this will give us more colors to work with.

Tertiary colors are obtained by mixing a secondary color with a primary color, in equal parts, thus obtaining even more hues to play with.

#### Analogous Colors

Analogous colors are tertiary colors that are next to each other on the color wheel. For instance Red, Orange and Vermilion (sometimes called Red Orange) are analogous colors.

They are good to create harmonious, almost-monocromatic compositions, since analogous colors are very common in nature.

#### Complementary Colors

Contrasting (complementary) color pairs were used in impressionism for their "eye-catching" character, they are created starting from the 3 primary colors (in screens: Red, Green, Blue), choosing one and combining the other two in a "secondary color".

Complementary colors are positioned on opposite sides of the color wheel. The color star (an alternative to the color wheel to represent colors) makes this "opposition" very easy to see:

![The color star shows how complementary colors are on opposite sides](./images/resources/color_star.svg){width=30%}

Some complementary color for screens (color addition) pairs are:

- **Red and Cyan**: Choose red, then green+blue gives cyan;
- **Green and Magenta**: Choose green, then red+blue gives magenta;
- **Blue and Yellow**: Choose blue, then red+green gives yellow.

While, talking about colors made by paint (color subtraction) we have tre following color pairs:

- **Magenta and Green:** Choose magenta, then yellow+cyan gives green;
- **Yellow and Purple:** Choose yellow, then magenta+cyan gives purple;
- **Cyan and Orange:** Choose cyan, then magenta+yellow gives you orange.

Complementary colors tend to attract the viewer's eye in the points of intersection of their hues.

#### Color Depth

Raster graphics use bits to represent the color of each single pixel, the amount of bits used for each pixel is known as *color depth* (or "bit depth").

The color depth used for our images can influence the performance and look of our game: more bits means more color choices, but also more memory occupied by those colors. Coupled with high resolutions, an image can easily (if uncompressed) weigh MB worth of memory.

Let's see the most common color depths used in history, first we start with a full-color reference image:

![Reference image that we will for bit depth comparison](./images/resources/bit_depth_reference.png){width=40%}

**1-Bit color:** Each pixel gets only 1 single bit, which means it's either black or white. This was used mostly in text-based systems, but some indie games manage to get great results out of just two colors.

![Reference image, quickly converted to 1-bit color depth](./images/resources/bit_depth_1bit.png){width=40%}

**2-Bit color:** Each pixel can select from a palette of 4 colors (usually fixed). This was used by CGA cards on the IBM, and usually the colors could be chosen from one of four available palettes: "white, black, light cyan, light magenta" or "yellow, black, light red, light green". The other two palettes are just a "low intensity variant" of the ones we've just seen.

![Reference image, converted to 2-bit color depth in CGA style](./images/resources/bit_depth_2bit.png){width=40%}

:::: trivia ::::
CGA was limited in the number of colors used, but due to a "defect" in the composite NTSC output (called "composite artifact colors" or "cross-color artifacting") these 4 colors could be used carefully to create completely new colors.
::::::::::::::::

**4-Bit color:** Here we start seeing 16 colors, usually chosen from a selection of fixed palettes. This was used by EGA cards on the IBM, as well as the Commodore 64 (along with "color cells").

![Reference image, converted to a 4-bit color depth in EGA style](./images/resources/bit_depth_4bit.png){width=40%}

**8-Bit color:** 256 colors chosen from a fully programmable palette (that's some luxury right there!). Used on VGA cards at low resolution.

![Reference image, converted to an 8-bit color depth](./images/resources/bit_depth_8bit.png){width=40%}

The previous image looks a lot like the reference image, save for some artifacting. Since at higher color depths we won't see much difference, let's just list the remaining ones:

- **16-Bit color:** Sometimes called "High Color", allows for up to 32768 colors with transparency, or up to 65536 without transparency.
- **24-Bit color:** Sometimes called "True Color", this is the most used color depth, allowing for over 16 million colors. Each red-green-blue channel has 256 possible values.
- **30-Bit color:** Sometimes called "Deep Color", this format allows for 10 bits per channel and over 1 billion colors on screen. Supported by many graphic cards and some high-end mobile phones.
- **48-Bit color:** This format allows for hundreds of thousands of billions of colors (if you want to read the number, it's around $281474976710656$). This is used by image editing software to avoid loss of data while working with colors.

#### Direct Color vs. Indexed Color

There are two main ways to represent color in digital images.

The first is "Direct Color", which usually allows 256 values (from 0 to 255 included) for each color channel (red, green and blue), for a total of over 16 Million colors.

Each single color is identified by its value, which can be a waste of space and memory when the image has few well-defined colors.

The second way to store images is with "indexed color": in this case a "palette" of colors is created and each pixel color refers to an index to such palette. This allows for smaller images, at the expense of the number of available colors. If you want to add a new color to the picture, first you need to add it to your palette if there is space.

#### Lossless Formats

There are a few ways to store information on a computer, either you store them raw, or you use some tricks to make such information occupy less space in your drive.

When it comes to storing information, lossless formats are usually uncompressed or make use of clever tricks to compress your images without losing information in the process.

In computer graphics, lossless formats include:

- JPEG 2000 (in its lossless form);
- Portable Network Graphics (PNG);
- Free Lossless Image Format (FLIF);
- PDF (in its lossless form);
- TARGA Files (Truevision TGA, with TGA file format);
- TIFF (in its lossless form).

#### Lossy Formats

When it comes to compressing information, the best way to store the least amount of information possible is to actually *not store them*. Lossy file formats get rid of some information present in the picture, in a more or less evident way (depending on the compression ratio) to lower the file size.

In computer graphics, lossy file formats include:

- JPEG (in its most common "lossy" form).

#### Transparency

Usually you need to have transparency in your artwork, for instance for your sprites. There are different ways to get transparency in your artwork, depending on the image format you're using and the support offered by the engine/framework you're using.

##### Alpha Transparency

This is the most common type of transparency available today: along with the usual Red-Green-Blue (RGB) channels, the image has an additional "Alpha" channel. Sometimes images with "Alpha Transparency" are also referred as "RGBA" images.

This allows to set the transparency precisely and allows for "partial transparency" too, which means that we are able to create shadows and semi-transparent surfaces.

The PNG format is one of the many image formats that supports alpha transparency.

##### Indexed Transparency

Normally used in GIF images, "Indexed Transparency" is the product of some limitation imposed in the format itself: you can only choose from a limited palette of colours to paint your picture.

If you want to have transparency in your picture, you will need to sacrifice a color and tell the format that such color is the "transparency color". In many images a very bright, evident color (like magenta) is used. Such color will not be painted, thus giving the transparency effect.

This also mean that we cannot make semi-transparent surfaces, since only that specific color will be fully transparent, and that's it.

### Texture Filtering {#texture_filtering}

Sometimes your images will need to be scaled or filtered to avoid annoying artifacts, in this small chapter we will see some filters and how they look.

#### Nearest Neighbor Filtering

{{placeholder}}

<!-- TODO: Nearest Neighbor Filtering -->

#### Bilinear Filtering

{{placeholder}}

<!-- TODO: Bilinear Filtering -->

#### Trilinear Filtering

{{placeholder}}

<!-- TODO: Trilinear Filtering -->

### General Tips

In this section we will take a look at some basic art tips that will give you some indication on how to create your own art for your very own game. This will be pointers to keep you going.

#### Practice, Practice, Practice...

As with all forms of art, the secret to "getting good" is practice. There's no way to avoid it, your first piece may been nice or flat out terrible, your next one will be better, and the one after that will be even better... Hard work always beats "talent" in the long run.

#### References are your friends

References are a tricky topic, some artists swear by it, others oppose them fiercely.

In my opinion, looking at a real-life version of what you want to draw can be one of the most useful things you can do when drawing. Even when drawing something that involved a huge amount of fantasy, having a reference can give you indications on shapes and sizes.

It doesn't have to be a one-to-one reference either, you can get ideas for your dragon from crocodiles and lizards, or even snakes!

#### Don't compare your style to others'

One of the most frustrating things that can happen when learning something new, is comparing yourself to another artist and saying "I should be able to draw like them".

Everyone has their own unique style, and you should work on what makes it unique, instead of comparing your style to others.

#### Study other styles

This ties a bit to the previous point, you should not compare to others, but you should also take some time to look at other people's work, find what you like about it and implement it into your own art style.

Looking around you can help you grow as an artist and aid you in the difficult seeking of your own art style.

#### Learn to deconstruct objects into shapes

Every complex object can be deconstructed into simpler shapes: circles, ellipses, triangles, rectangles, squares...

You can use such simple shapes, overlapping them to create a skeleton of your subject (living or not), so that you can draw it in an easier way: with the layer system introduced by huge part of the current drawing applications, you don't have to do precision work when it comes to removing such skeleton.

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


### Virtual Resolution {#VirtualRes}

There are times where having the crispest graphics around is not a good idea, for instance in Pixel-Art games.

Times where it's a better idea to keep your game low-resolution, be it for a matter of performance (your first games won't be extremely optimized and will be slow even at low map sizes and resolutions) or to keep your pixel-art crisp and "pixelly".

This clashes with the continuous push towards higher resolutions, which can mess up your game in a variety of ways, like the following ones.

If the game is forced in windowed mode, you'll have a problem like the following:

![Windowed Game Example - A 640x480 game in a 1920x1080 Window](./images/resources/virtual_res_fail_1.svg){width=60%}

The game window is way too small and hard to see, and can get even smaller if the *[HUD~\[g\]~](#gl_hud)* takes even more space out of the window. This can be mitigated by calculating the position of each element in comparison to the window size, this although can result in items too small (or too big if downscaling), like the following HUD example.

![Fullscreen Game Example - Recalculating items positions according to the window size](./images/resources/virtual_res_fail_2.svg){width=60%}

This is where **virtual resolution** comes into play, the frame is rendered in a virtual surface which operates at a "virtual resolution" and the whole frame is drawn and upscaled (or downscaled) to the screen, without having to recalculate anything in real time (thus making the game faster and lighter).

![Fullscreen Game Example - Virtual Resolution](./images/resources/virtual_res.svg){width=60%}

The items get scaled accordingly and there is no real need to do heavy calculations. Virtual Resolution allows for different kinds of upscaling, with or without filtering or interpolation, for instance:

- **No filtering** - Useful for keeping the "pixeliness" of your graphics, for instance in pixel-art-based games. It's fast.;
- **Linear, Bilinear, Trilinear Filtering** - Gives a more soft look, slower;
- **Anisotropic Filtering** - Used in modern 3D games, highest quality but also among the slowest, it is usually used when rendering sloped surfaces (from the player's point of view).

For more details, check the [filtering](#texture_filtering) section.

### Using limited color palettes

{{placeholder}}

<!-- TODO: Talk about how to make your art consistent using palettes, talk about DawnBringer's palette -->

### Dithering

Dithering (usually called "Color Quantization") is a technique used to give the illusion of a higher "color depth" when you're using a limited palette of colors.

The usage of dithering introduces patterns into the image when the pixels are visible, if instead the pixels are small enough the pattern will look like a new color, without actually introducing a new color into the palette.

Using two different levels of blue, we can use a dithering pattern to obtain a new tone of blue, like the following:

![Dithering example](./images/resources/dithering.png){width=50%}

It's possible to study how your palette reacts to dithering using a dithering table, this will give you an idea of the colors available via dithering.

You can see a simple example of a dithering table here:

![Dithering Table Example](./images/resources/dithering_table.png){width=50%}

You can see the palette colors on the top row and the left column, then you can see how dithering (in this case a simple checkerboard pattern) allows for different colors to pop out.

There are different dithering patterns, that allow for different type of colors, intensity and patterns:

![Some more dithering examples](./images/resources/dithering_patterns.png){width=50%}

### Layering and graphics

There are some things that should be kept in mind when drawing layers for your game, here we talk about some key points about layering and graphics.

#### Detail attracts attention

When it comes to games, it's easy to get too excited and craft your work with the highest amount of detail possible, but there is a problem: detail tends to attract players' attention.

If you put too much detail in the background, you're going to distract them from the main gameplay that happens in the foreground, which can prove dangerous: the graphics can get messy and you can even get to the point of not being able to distinguish platforms from the background.

So a golden rule could be:

> Use high detail in the foreground, gameplay-heavy elements - use less detail in the backgrounds

A good idea to make the background a bit less detailed is using blurring, which allows to keep the overall aesthetic but makes it look "less interesting" than what's in the foreground.

This doesn't mean the background should be devoid of detail, just don't overdo it.

#### Use saturation to separate layers further

Bright colors attract attention as much as detail does, so a good idea is making each background layer more "muted" (color-wise) than the ones in foreground.

The main technique to make backgrounds more muted is lowering saturation, blending the colors with grey: this will make the background less distracting to the player.

So another rule can be written as:

> Layers farther away should have lower color saturation than the layers closer to the camera

#### Movement is yet another distraction

As detail and saturation are due to attract attention from the player, movement is another one of those "eye-catchers" that can make the gameplay more chaotic and difficult for the player.

Small amounts of movement are ok, but fully-fledged animations in the background will prove distracting.

Let's take note of rule number 3 then:

> Try to avoid movement in the background

#### Use contrast to your advantage {#contrast_to_your_advantage}

Complementary colors tend to attract a lot of attention in the points of intersection of their hues.

If backgrounds feature complementary colors, it may distract the player from the main gameplay.

Our rule number four should then be:

> Keep backgrounds low-contrast to avoid distracting players

Also the opposite rule may apply:

> Keep the main gameplay elements contrasting, so to attract the attention towards them

An orange-robed character will be easier to follow on a blue-ish background, for instance.

#### Find exceptions

Nothing is ever set in stone, and no rules should keep you from playing a bit outside the box and fiddling with some exceptions to a couple rules.

### Palette Swapping

Palette swapping is a technique mostly used in older videogames, where an already-existing graphic (like a player character's sprite) is reused with a different palette (combination of colors).

This palette swap makes the new graphic quite recognizably distinct from the original graphic. This technique was normally used to tell apart first and second player.

:::: trivia ::::
A prime example of this is the videogame that (re)started it all: Super Mario Bros. Mario and Luigi are exactly the same sprite, but Luigi uses a different palette.
::::::::::::::::

Some other videogames use palette swapping to indicate their status (like using a green or purple-based palette to indicate the "poisoned" status), or indicate a difference in their statistics (like a red-based palette to indicate an enhanced attack statistic), in other occasions different palettes are used to distinguish stronger versions of the same enemy.

Other franchises, like Pokémon, use palette swaps to introduce "special versions" of some entity (in the case of Pokémon, a shiny pokemon).

Palette Swapping can be used in more creative ways, though. Going back to Super Mario Bros. you can see that the clouds and the bushes in the levels are exactly the same graphic, just with a different palette. Same goes for the underground bricks and the overworld bricks: they just have a different color.

### Pixel Art

{{placeholder}}

<!-- TODO: talk about pixel art, etc...-->

#### What pixel art is and what it is not

{{placeholder}}

<!-- TODO: Talk how pixel art is a mean, and it's more than just a "style" -->

#### Tools

When it comes to pixel art, your most used tools will be:

- **Pencil:** Its hard borders are ideal for base colors and outlines, allowing per-pixel editing without worrying about blur radius;
- **Brush:** The brush tool's soft borders are great for shading, in case you want to go for a more "hi-res" look;
- **Paint Bucket:** This is great for filling outlined ares with a base color. This allows also to create "zones" that help you distinguish between different parts of a sprite;
- **Eraser:** Great tool for removing stray pixels! Just make sure the eraser is set to delete the whole pixel and not just "make it more transparent looking".

Opacity options for tools are great for shading: you can take the same color and blend it with other existing colors, or make it stronger by going through the same area more than once. This makes the process more akin to real painting.

#### Layers

As with all other drawing styles, layers are your best friends in pixel art too. Just make sure to start with a transparent layer, since some drawing programs (for memory and CPU efficiency) won't let you easily add a "transparent background layer" afterwards.

Layers allow you to work on something new without affecting what's already "ready", as well as separate parts of a character (arms, legs, torso and head, for instance).

You can always "flatten the image" (merge all the layers into one) later.

#### Sub-pixel animation

{{placeholder}}

<!-- TODO: Style of animation where the "outlines" don't move, but the color change in the internal pixels can give illusion of sub-pixel movement -->

### Normal Mapping

If you want your 2D game to look amazing, you can't escape shaders. One of the ways to use shaders is calculating light, but since we're in a 2D environment, we don't have "normal vectors" to use to calculate how the light interacts with our 2D objects.

Enter normal maps: these are sprites that "map" their color channels to the direction of the "normal vector": this means that communicate direction with color. Awesome, isn't it?

::: trivia :::
If you're curious, normal maps usually have this color to direction mapping:

- x (which can go from -1 to +1) is mapped to the red channel (which can go from 0 to 255)
- y (which can go from -1 to +1) is mapped to the green channel (which can go from 0 to 255)
- z (which can go from 0 to -1) is mapped to the blue channel (which can go from 128 to 255)
::::::::::::::

This is an example of a texture, along with a possible normal map:

![Example of a normal map](./images/resources/normal_mapping_1.jpg){width=50%}

There are many ways to get a normal map: you can try to get a program to generate one for you, make the object in a 3D modeler and extract the normal map from there, or just draw it by hand.

If you choose the last option some tools, like Aseprite, have a "normal mapping" mode that shows you this special color picker:

![Aseprite's normal mapping color picker (both in its normal and discrete versions)](./images/resources/normal_mapping_2.png){width=40%}

Just imagine this color picker like a "3D sphere" and pick the color of the face of the "3d surface" you're trying to draw.

::: pitfall :::
Be careful with your shaders, some may expect one or more of your channels to be "flipped".
:::::::::::::::

#### A simple example

Let's take a simple box, with no shading, like the one below:

![A box that will be used to show how normal maps influence light](./images/resources/Box_No_Light.png){width=25%}

Now we'll shine a light on the box, without any normal map: this will happen twice:

In the first example (left) the light will be a round gradient that will come from the top right corner of the image, while in the second example the light will be a bit stronger and coming from the top left corner. This is the result.

![How the lack of normal mapping makes lighting look artificial](./images/resources/No_Normal_Map.png){width=35%}

Now we'll draw the simplest normal map possible: just filling the 3 faces of the box that we can see with the (kind of) corresponding colors from our "3d sphere" (the normal mapping color picker), we can see the result is very different

![How normal mapping changes lighting](./images/resources/Normal_Map_Rough.png){width=60%}

Now let's make something a bit more detailed, by highlighting the faces of the cross-braces on the sides of the box, the way they're lit it's again different:

![A more detailed normal map results in better lighting](./images/resources/Normal_Map_Detailed.png){width=60%}

You can get as detailed as you want, but remember that it may have some performance impact if you go overboard with many sprites.

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
