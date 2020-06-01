\null\clearpage

Creating your resources
=======================

\epigraph{Art is not what you see, but what you make others see.}{\textit{Edgar Degas}}

Graphics {#GraphicsResources}
---------

### Some computer graphics basics

\placeholder
<!-- TODO -->

#### Color Depth

\placeholder
<!-- TODO -->

#### True Color vs. Indexed Color

There are two main ways to represent color in digital images.

The first is "True Color", which allows 256 values (from 0 to 255 included) for each color channel (red, green and blue), for a total of over 16 Million colors.

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

### Texture Filtering

Sometimes your images will need to be scaled or filtered to avoid annoying artifacts, in this small chapter we will see some filters and how they look.

#### Nearest Neighbor Filtering

\placeholder
<!-- TODO -->

#### Bilinear Filtering

\placeholder
<!-- TODO -->

#### Trilinear Filtering

\placeholder
<!-- TODO -->

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

\placeholder


### Virtual Resolution {#VirtualRes}

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

For more details, check the [filtering](#filtering) section.

### Using limited color palettes

\placeholder

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

- **Red and Cyan**: Choose red, then green+blue gives cyan;
- **Green and Magenta**: Choose green, then red+blue gives magenta;
- **Blue and Yellow**: Choose blue, then red+green gives yellow.

*Remember that we're talking about the RGB model of colors produced by **light** (color addition), not the traditional color wheel*

While, talking about colors made by paint (color subtraction) we have tre following color pairs:

- **Magenta and Green:** Choose magenta, then yellow+cyan gives green;
- **Yellow and Purple:** Choose yellow, then magenta+cyan gives purple;
- **Cyan and Orange:** Choose cyan, then magenta+yellow gives you orange.

These colors tend to attract a lot of attention in the points of intersection of their hues, distracting the player from the main gameplay.

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

A prime example of this is the videogame that (re)started it all: Super Mario Bros. Mario and Luigi are exactly the same graphic, but Luigi uses a different palette.

Some other videogames use palette swapping to indicate their status (like using a green or purple-based palette to indicate the "poisoned" status), or indicate a difference in their statistics (like a red-based palette to indicate an enhanced attack statistic), in other occasions different palettes are used to distinguish stronger versions of the same enemy.

Other franchises, like Pokémon, use palette swaps to introduce "special versions" of some entity (in the case of Pokémon, a shiny pokemon).

Palette Swapping can be used in more creative ways, though. Going back to Super Mario Bros. you can see that the clouds and the bushes in the levels are exactly the same graphic, just with a different palette. Same goes for the underground bricks and the overworld bricks: they just have a different color.

### Pixel Art

\placeholder

#### What pixel art is and what it is not

\placeholder

#### Sub-pixel animation

\placeholder

<!-- TODO: talk about pixel art, etc...-->

### Tips and Tricks

This section contains various tips and tricks used by artists to create certain effects inside video games, demonstrating how sometimes something really simple can have a great effect on the game experience.

#### Creating "Inside rooms" tilesets

In many cases, when dealing with tile-based games, we need to create a tileset that is good to represent "inside" environments, like a basement, a cave or the inside of a building. A simple way to reach that goal is creating a set of black and transparent tiles that can be overlaid on another tileset, like the following:

![Example of black and transparent tileset used in "inside rooms"](./images/resources/inside_tileset.png){width=30%}

Such tiles can then be overlaid onto something like the following:

![Example of incomplete "inside room"](./images/resources/inside_example_1.png){width=40%}

And we obtain the following result:

![Example of "inside room" with the black/transparent overlay](./images/resources/inside_example_2.png){width=40%}

\placeholder

Sounds And Music
----------------

\placeholder

<!-- TODO: Talk about the importance of good sound quality, introduce people to chip tunes and tools to create music and sounds , talk about the importance of having areas where you have to apply a certain effect or it will happen on the whole level -->


### Some audio basics

\placeholder
<!-- TODO -->

#### Sample Rate

Differently from Analog Audio, which is continuous (as in has an infinite amount of detail), Digital Audio is a stream of numbers (ones and zeros) that is "discrete" in nature. That means that we blast these numbers thousands of times a second to be able to build a decent sounding sound.

The number of times we record such numbers from our digital microphone (as well as the number of times we blast such numbers back from our speakers) is called **sample rate** and it is measured in *Hz*.

![Graphical Representation of Sample Rate (44.1KHz)](./images/resources/Sample_Rate_44100.png){width=60%}

In normal CD-Audio, we have a sample rate of 44100 Hz, which means that we recorded a sample 44100 times in a single second.

When making our game's audio, we should always stay around such value, since going lower would make the audio sound worse, since we lower the amount of information the audio itself has.

![Graphical Representation of Sample Rate (8KHz)](./images/resources/Sample_Rate_8000.png){width=60%}

Also we should avoid using weird sample rates, 44.1KHz (or 44100 Hz if you prefer) is a "magic value" that guarantees the most compatibility.

#### Bit Depth

Along with sample rate, there is another value in audio that expresses its quality: the bit depth.

The bit depth defines how many "volume values" we can have in a single sample, which shapes the quality of the sound in a different way than sample rate.

If our audio has a 1-bit sample rate, each sample will have only 2 values:

- **0:** Mute
- **1:** Blast at full volume

Which reduces the quality of the audio.

Usually audio has a 16 Bit depth, but more modern systems make use of 24 Bits or even 32 Bits.

#### Lossless Formats

As with graphics, there are audio formats that allow you to store uncompressed information, as well as compressed (but without losses) sounds. The most common lossless audio formats include:

- WAV;
- AIFF;
- FLAC.

#### Lossy Formats

As with graphics, there are also "lossy formats" that allow us to store information in even less space by getting rid of information that is well outside our hearing spectrum, for instance. Some of the most known are:

- Mpeg Layer 3 (MP3);
- OGG Vorbis;
- Windows Media Audio (WMA);
- Advanced Audio Codec (AAC).

#### Clipping

Clipping is a phenomenon when you're trying to output (or record) a wave that exceeds the capacity of your equipment. As a result, the wave is "clipped" (cut) and there is a very audible distortion.

You surely heard clipping in audio before, usually when people scream on a low-quality microphone and the audio gets distorted.

The best way to repair clipping is to re-record the audio completely, although some tools can help in case you absolutely cannot re-record the audio.

Also you should be wary of clipping, because there may be cases where it damages your audio equipment.

### Digital Sound Processing (DSP)


Let's think about a simple situation: we want to play a "walk" sound effect: every time our character's foot hits the ground, we play a "step" sound effect.

If we play the same sound over and over, it will become boring really quickly, breaking the immersion. An idea could be saving different sounds for a footstep and then every time the player takes a step, a random footstep sound will be played.

This solves the problem, at a cost: we need to use more memory to keep track of such sounds, multiply that for tens of sound effects and the game can easily run out of memory.

An alternative solution could be using DSP: editing the sound sample in real time to add more variety and depth while saving memory, the trade-off would be CPU time, but it's an acceptable deal.

#### Reverb

When you take a stroll on a sidewalk, you have a certain "openness" on the footstep sounds you hear, but that surely changes if you're walking with hard shoes on a hard floor inside of a small cave. You can hear a lot of reverb and echo at every single step.

Reverb is the first of the sound effects that we encounter in our journey: it allows to give more depth to our sound effects, making it sound like we're inside of a small cave or a very large room.

#### Pitch Shift

A way to give more variety to a sound effect without much work is using pitch shift to make our sound a bit higher or lower, randomly, so that each step is slightly different from the other: this way our ears will get less tired of hearing said sound effect.

Pitch shifting must be used with caution, since abusing it will distort the sound effect and break the immersion in our game.

Another example of pitch shift is used in racing games, where the car roar is pitch-shifted up or down according to the acceleration given to the car.

#### Filtering

Another sound effect we can use is filtering, which are divided in 3 main sections, according to the frequency they "allow to pass through":

- **Low Pass Filter:** This filter allows low frequencies to pass through unfiltered, while the frequencies higher than a defined threshold will be cut. This allows for effects where the bass is unaltered but higher frequencies are cut away;
- **High Pass Filter:** Opposite of the previous filter, this filter allows high frequencies to pass through unaltered while the frequencies lower than a defined threshold will be cut;
- **Band Pass Filter:** A combination of the two previous filters, this filter let's through all the frequencies between two defined threshold values. This allows for more interesting effects like a music sounding through an old radio.

An interesting example is when an explosion happens near the player, in that case the "stun" effect is given by using a low pass filter on an explosion sound (which makes it sound really low and muffled), eventually a slowdown is applied and a secondary sound effect of a very high pitch sound is added (something similar to what you hear when your ears are ringing).

#### Doppler Effect

To give more depth to your sound effects, you can use pitch shift to create the "Doppler Effect" that we hear in real life when a police car passes by: when the car approaches us the pitch is higher, when the car is in front of us we hear the siren as it should be, and when the car passes us we hear a lower pitched version of the siren sound effect.

The Doppler effect can be really useful when applied to car racing games again, when we overtake one of our opponents (or one of our opponents overtakes us) using the doppler effect can help the player feel more "immersed" in the experience.

The doppler effect would actually apply to light too, but we would need to have something travelling at a really high speed or said object to be really far away (like a planet).

### "Swappable" sound effects

Back to our walking example, an idea to increase the variety of sound effects at our disposal would be keeping a list of "swappable sounds": sounds that are still part of the class we're considering, but are radically different.

For instance we could have different walking sounds for different floors, so that walking on grass and walking on a stone pavement will be different. In this case it would be useful to make the sounds configurable and give the sound manager the chance to inspect what type of floor we're walking on.

An example of "swappable sound effects" configuration is given in the following file, which is written in YAML:

```{.yaml caption="Example of swappable sound effects"}
footsteps:
  grass:
    - grasswalk1.wav
    - grasswalk2.wav
  stone:
    - stonewalk1.wav
    - stonewalk2.wav
  metal:
    - metalstep1.wav
    - metalstep2.wav
```

Making a configuration file instead of hard-coding the elements allows for easy extensibility and modding, which everyone loves (See [Designing entities as data](#entitiesasdata)).

### Some audio processing tips

\placeholder
<!-- TODO -->

#### Prefer cutting over boosting

Sometimes we may find our audio samples lacking that "punch" they would need, the first idea we may have would be to use a "bass boost" filter to make the low frequencies more prominent. Most of the time, this is not a good idea, since boost filters can create artifacts.

It's better to cut the higher frequencies instead, and eventually boost the entire volume of the sample during mixing. This way the nature of the sample doesn't get tainted by boosting, and we obtain the result we wanted.

Fonts
-------

### Font Categories

Before starting with fonts and the ways they can be integrated in your game, we should start with some definitions and categorizing fonts by their own characteristics.

#### Serif and Sans-Serif fonts

In typography, *serifs* are small strokes that get attached to larger strokes in a letter (or symbol) of certain fonts. The font families that make use of serifs are called **serif fonts** or **serif typefaces**.

Serif fonts look more elegant and give a "roman" feeling (in fact, serif fonts are also called *roman* typefaces) and are good for games that take place in historical settings or need a semblance of pretend "historical importance" (in their own world).

![Example of a serif font (DejaVu Serif)](./images/resources/serif_font.png){width=30%}

Serif fonts look better on paper and could come out as a bit harder to read on screens. A famous serif font is Times New Roman.

On the opposite side, we have **sans-serif fonts**, where such small strokes are absent. Sans-Serif fonts seem easier to read on screens and look simpler, but they don't look as good on paper, when long text bodies are involved.

![Example of a sans-serif font (DejaVu Sans)](./images/resources/sans_serif_font.png){width=30%}

A famous sans-serif font is Arial.

### Proportional and Monospaced fonts

The majority of fonts used today are **proportional**, where each letter occupies its own space proportional to its own width. Examples of proportional fonts are Times New Roman and Arial.

![Example of a proportional font (DejaVu Serif)](./images/resources/proportional_font.png){width=30%}

Notice the difference in width between certain pairs of letters, like "i" and "o" or "a" and "l".

Proportional fonts are good for general text that don't have any particular constraint.

On the opposite side, there are **monospaced** fonts, also called **fixed-width** fonts. In these font families, each letter occupies the same amount pre-defined width.

![Example of a monospaced font (Inconsolata)](./images/resources/monospace_font.png){width=30%}

Again, notice how all letters occupy the same horizontal space.

Monospaced fonts are used for computer texts, coding and ascii-art. Examples of monospaced fonts are Courier New and Inconsolata.

### Using textures to make text

\placeholder

<!-- TODO: Good and fast, but issues with scaling and resizing -->

### Using Fonts to make text

\placeholder

<!-- TODO: Good and resizes well, but can be slower cause the text needs to be rendered -->

Shaders
--------

### What are shaders

Shaders are technically small programs that (usually) run inside your Graphics Card (GPU). In gaming they are usually used for post-processing and special effect, allowing to free the CPU from a lot of workload, using the specialized GPU capabilities.

Shaders can be classified in different groups:

- **2D Shaders:** These shaders act on textures and modify the attributes of pixels.
    - **Pixel (Fragment) Shaders:** Used to compute color and other attributes relating to a single output pixel. They can be used for blur, edge detection or enhancement and cel/cartoon shading.
- **3D Shaders:** These shaders act on 3D models or other geometry.
    - **Vertex Shaders:** These shaders are run once per vertex given to the GPU, converting the 3D position in virtual space to the 2D coordinates of your screen. These shaders cannot create any new geometry.
    - **Geometry Shaders:** These shaders can create new primitives, like points or triangles.
    - **Tessellation Shaders:** These shaders allow to divide simple meshes into finer ones at runtime, this allows the meshes closest to the camera to have finer details, while the further ones will be less detailed.
    - **Primitive Shaders:** Akin to the computing shaders, but have access to data to process geometry.
- **Computing Shaders:** These shaders are not limited to graphics, but are related to the world of GPGPU (General Purpose computing on GPU), these can be used to further stages in animation or lighting.

### Shader Programming Languages

There are numerous programming languages, depending on the platform and libraries you are using to program your game.

If you are using OpenGL, you should use the official OpenGL Shading Language, called **GLSL**.

If you are using Direct3D, you should instead use the "High Level Shader Language", also called **HLSL**.

If instead you want to use Vulkan, you will need to use the **SPIR-V** (Standard Portable Intermediate Representation) format, but the good news is that (at the time of writing) you can convert your GLSL code into SPIR-V and use it with Vulkan.

Modern engines, like Unity and Unreal Engine also include GUI node-based editors that help you create new shaders by using directed graphs, without using any code.

### The GLSL Programming Language

\placeholder

<!-- TODO: Teach some basic GLSL -->

### Some GLSL Shaders examples

\placeholder

<!-- TODO: Add some simple 2D fragment shaders examples -->
