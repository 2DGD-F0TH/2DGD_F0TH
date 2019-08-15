\null\clearpage
Creating your resources
=======================

Graphics
---------

### Virtual Resolution

There are times where having the crispest graphics around is not a good idea, for instance in Pixel-Art games.

Times where it's a better idea to keep your game low-resolution, be it for a matter of performance (your first games won't be extremely optimized and will be slow even at low map sizes and resolutions) or to keep your pixel-art crisp and "pixelly".

This clashes with the continuous push towards higher resolutions, which can mess up your game in a variety of ways, like the following ones.

If the game is forced in windowed mode, you'll have a problem like the following:

![Windowed Game Example - A 640x480 game in a 1920x1080 Window](./images/resources/virtual_res_fail_1.pdf){width=60%}

The game window is way too small and hard to see, and can get even smaller if the HUD~[g]~ takes even more space out of the window. This can be mitigated by calculating the position of each element in comparison to the window size, this although can result in items too small (or too big if downscaling), like the following HUD example.

![Fullscreen Game Example - Recalculating items positions according to the window size](./images/resources/virtual_res_fail_2.pdf){width=60%}

This is where **virtual resolution** comes into play, the frame is rendered in a virtual surface which operates at a "virtual resolution" and the whole frame is drawn and upscaled (or downscaled) to the screen, without having to recalculate anything in real time (thus making the game faster and lighter).

![Fullscreen Game Example - Virtual Resolution](./images/resources/virtual_res.pdf){width=60%}

The items get scaled accordingly and there is no real need to do heavy calculations. Virtual Resolution allows for different kinds of upscaling, with or without filtering or interpolation, for instance:

- **No filtering** - Useful for keeping the "pixeliness" of your graphics, for instance in pixel-art-based games. It's fast.;
- **Linear, Bilinear, Trilinear Filtering** - Gives a more soft look, slower;
- **Anisotropic Filtering** - Used in modern 3D games, highest quality but also among the slowest, it is usually used when rendering sloped surfaces (from the player's point of view).

### Using limited color palettes

<!-- TODO: Talk about how to make your art consistent using palettes, talk about DawnBringer's palette -->

### Dithering

Dithering (usually called "Color Quantization") is a technique used to give the illusion of a higher "color depth" when you're using a limited palette of colors.

The usage of dithering introduces patterns into the image when the pixels are visible, if instead the pixels are small enough the pattern will look like a new color, without actually introducing a new color into the palette.

Using two different levels of blue, we can use a dithering pattern to obtain a new tone of blue, like the following:

![Dithering example](./images/resources/dithering.png)

It's possible to study how your palette reacts to dithering using a dithering table, this will give you an idea of the colors available via dithering.

You can see a simple example of a dithering table here:

![Dithering Table Example](./images/resources/dithering_table.png)

You can see the palette colors on the top row and the left column, then you can see how dithering (in this case a simple checkerboard pattern) allows for different colors to pop out.

There are different dithering patterns, that allow for different type of colors, intensity and patterns:

![Some more dithering examples](./images/resources/dithering_patterns.png)

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

The main technique to make backgrounds more mutes is lowering saturation, blending the colors with grey: this will make the background less distracting to the player.

So another rule can be written as:

> Layers farther away should have lower color saturation than the layers closer to the camera

#### Movement is yet another distraction

As detail and saturation are due to attract attention from the player, movement is another one of those "eye-catchers" that can make the gameplay more chaotic and difficult for the player.

Small amounts of movement are ok, but fully-fledged animations in the background will prove distracting.

Let's take note of rule number 3 then:

> Try to avoid movement in the background

#### Use contrast to your advantage

Contrasting (complementary) color pairs were used in impressionism for their "eye-catching" character, they are created starting from the 3 primary colors (in screens: Red, Green, Blue), choosing one and combining the other two in a "secondary color".

Some complementary color pairs are:

- **Red and Cyan**: Choose red, then green+blue gives cyan
- **Green and Magenta**: Choose green, then red+blue gives magenta
- **Blue and Yellow**: Choose blue, then red+green gives yellow

*Remember that we're talking about the RGB model of colors produced by **light**, not the traditional color wheel*

These colors tend to attract a lot of attention in the points of intersection of their hues, distracting the player from the main gameplay.

Our rule number four should then be:

> Keep backgrounds low-contrast to avoid distracting players

#### Find exceptions

Nothing is ever set in stone, and no rules should keep you from playing a bit outside the box and fiddling with some exceptions to a couple rules.

### Palette Swapping

\placeholder

<!--TODO: Talk about how to extend the game's graphics using palette swapping -->

### Pixel Art

\placeholder

<!-- TODO: talk about pixel art, etc...-->

Sounds
------

\placeholder

<!-- TODO: Talk about the importance of good sound quality, introduce people to chip tunes and tools to create music and sounds -->
