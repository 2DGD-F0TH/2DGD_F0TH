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
This is due to how light and paint work differently. It is said that "lights are additive" and "paints are subtractive": if you mix Magenta, Yellow and Cyan paints using minuscule dots (like in printing), you will see that Red, Green and Blue are formed. This is obviously an idealization, the colors you'd actually obtain by mixing paints by hand would be closer to Orange, Purple and Dark Green. In short, this is how ink-jet printing works (also a Black component is added, thus forming the CMYK model).
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

Sometimes we need to represent transparency, in that case we need an extra pair of bytes to do so. In this case we talk about the "RGBA system" (Red-Green-Blue-Alpha).

Colors are represented by a 4-tuple: $(RRR, GGG, BBB, AAA)$, where each channel can take a value from $0$ to $255$ (or $00$ to $FF$ in hexadecimal).

##### HSV Representation

Another way to represent colors is using the HSV system (Hue, Saturation, Value). Sometimes this system is also called HSB (Hue, Saturation, Brightness).

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

They are good to create harmonious, almost-monochromatic compositions, since analogous colors are very common in nature.

#### Complementary Colors

Contrasting (complementary) color pairs were used in impressionism for their "eye-catching" character, they are created starting from the 3 primary colors (in screens: Red, Green, Blue), choosing one and combining the other two in a "secondary color".

Complementary colors are positioned on opposite sides of the color wheel. The color star (an alternative to the color wheel to represent colors) makes this "opposition" very easy to see:

![The color star shows how complementary colors are on opposite sides](./images/resources/color_star.svg){width=30%}

Some complementary color for screens (color addition) pairs are:

- **Red and Cyan**: Choose red, then green+blue gives cyan;
- **Green and Magenta**: Choose green, then red+blue gives magenta;
- **Blue and Yellow**: Choose blue, then red+green gives yellow.

While, talking about colors made by paint (color subtraction) we have three following color pairs:

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

![Indexed transparency takes a color and "marks it" as transparent](./images/resources/indexed_transparency.png){width=40%}

This also mean that we cannot make semi-transparent surfaces, since only that specific color will be fully transparent, and that's it.

#### Texture Filtering {#texture_filtering}

Sometimes your images will need to be scaled or filtered to avoid annoying artifacts, in this small chapter we will see some filters and how they look.

##### Nearest Neighbor Filtering

{{placeholder}}

<!-- TODO: Nearest Neighbor Filtering -->

##### Bilinear Filtering

{{placeholder}}

<!-- TODO: Bilinear Filtering -->

##### Trilinear Filtering

{{placeholder}}

<!-- TODO: Trilinear Filtering -->

#### What is a sprite?

While reading this book, you might already have seen the term "sprite" being used a lot, but what is a sprite?

A sprite is a 2D image (or animation), that is composited with other sprites or textures into a larger scene. In a game, usually they are composed by at least 2 elements:

- A texture (which represents the image itself);
- A position (usually represented with an $(x,y)$ pair).

In some media, you may find other terms that are used as synonyms of "sprite", like:

- Player Graphics/Missile Graphics: These terms come from the Atari line of computers and consoles;
- Objects (or OBJs): These terms usually come from the Nintendo line of consoles, for instance when referring to the memory region where sprite attributes are stored; in that case it was called OAM (Object Attribute Memory);
- MOBs (or Movable Object Block): Coming from the MOS Technology data sheets. Strangely enough Commodore, the main user of MOS products, used the term "sprite" instead.
