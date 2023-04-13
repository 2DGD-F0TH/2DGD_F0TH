### Using limited color palettes

{{placeholder}}

<!-- TODO: Talk about how to make your art consistent using palettes, talk about DawnBringer's palette -->

#### Dithering

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

#### Palette Swapping

Palette swapping is a technique mostly used in older video games, where an already-existing graphic (like a player character's sprite) is reused with a different palette (combination of colors).

This palette swap makes the new graphic quite recognizably distinct from the original graphic. This technique was normally used to tell apart first and second player.

:::: trivia ::::
A prime example of this is the video game that (re)started it all: Super Mario Bros. Mario and Luigi are exactly the same sprite, but Luigi uses a different palette.
::::::::::::::::

Some other video games use palette swapping to indicate their status (like using a green or purple-based palette to indicate the "poisoned" status), or indicate a difference in their statistics (like a red-based palette to indicate an enhanced attack statistic), in other occasions different palettes are used to distinguish stronger versions of the same enemy.

Other franchises, like Pokémon, use palette swaps to introduce "special versions" of some entity (in the case of Pokémon, a shiny Pokemon).

Palette Swapping can be used in more creative ways, though. Going back to Super Mario Bros. you can see that the clouds and the bushes in the levels are exactly the same graphic, just with a different palette. Same goes for the underground bricks and the overworld bricks: they just have a different color.
