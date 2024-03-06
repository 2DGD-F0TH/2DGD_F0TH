General Purpose
---------------

### I-Frames

I-Frames (also known as "invincibility frames") is a term used to identify that period of time after being hit, where the character either flashes or becomes transparent and is immune to damage.

This mechanic can be seen as "giving the player an advantage" but instead it has deeper roots into "fairness" than "difficulty management", let's see why.

Let's assume that our main character has 100 health points, and touching an enemy deals 5 points of damage. In absence of I-Frames, this would translate into 5 points of damage every frame, which would in turn come out between $5 \cdot 30 = 150$ and $5 \cdot 60 = 300$ points of damage per second (at respectively 30 and 60fps).

The average human reaction time is around 1 second, this would mean that touching an enemy would kill us before we even realize we are touching such enemy.

Checking if we're still colliding with an enemy after receiving damage is not a good strategy, since that would allow the player get only one point of damage from a boss, and then carefully stay inside the boss's hitbox while dealing damage to the enemy. Thus allowing the player to exploit the safeguard.

Giving a brief period (usually between 0.5 and 2 seconds) of invincibility after being hit, allows the player to understand the situation, reorganize their strategy and take on the challenge at hand. After the invincibility period, the player will take damage again, patching the exploit we identified earlier.

I-Frames can be easily implemented via timers, in a way similar to the following:

```{src='developing_mechanics/iframes' caption='Example of I-Frames Implementation'}
```

:::: tip ::::
Remember: feedback is important! You need to let the player know when they are invincible due to i-frames. This can be done by making the player semi-transparent, flashing or anything else that can indicate a "different status".
:::::::::::::

### Tilemaps

Tilemaps are a really interesting abstraction that allows us to draw maps by using pre-made "tiles" instead of having to draw them "pixel-by-pixel".

![Example of a tileset and a tilemap drawn with it [^jawbreaker]](./images/developing_mechanics/tilemap.png){width=65%}

This also allows us to have a new coordinate system that works "using tiles", which could be preferable than single pixels (since we may put properties on our tiles, like a "solid" property for collision detection).

Another advantage of tilemaps is the ability to use a small texture to draw gigantic maps without adding much data in memory (tilemaps are a fantastic example of the "Flyweight Pattern").

#### Rectangular Tilemaps

Rectangular tilemaps are the most commonly used tile maps in game development: it's easy to translate back and forth between "screen pixels" and "tiles", and if the tilesets are well-made everything looks seamless.

This has the advantage of using less memory (we need to save only the tileset, plus a few coordinates and pointers), thus making our game perform better.

:::: trivia ::::
Super Mario Bros. uses maps that are based on square tiles. Even the pipes are tiles: this allows the game to have variable lengths of pipes without increasing the number of tiles.
::::::::::::::::

{{placeholder}}
<!-- TODO: Quick talk about rectangular tilemaps -->

#### Hexagonal Tilemaps

Sometimes you may want to underline a "tabletop" game feel, in that case a hexagonal tilemap (sometimes called "hexmap") may be a great idea (at the cost of more complicated algorithms).

![Simple structure of a hexmap](./images/developing_mechanics/hexmap.svg){width=40%}

Hexmaps allow for a different kind of movement (the player can move to up to 6 directions, instead of 4), which makes for an interesting remix of the classic tile-based mechanics.

To be able to work with tilemaps, we need to get acquainted with the concept of "outer circle" of a polygon, which is the circle that intersects the edges of a polygon. In the case of our hexagon

![The outer circle or an hexagon](./images/developing_mechanics/hex_circle.svg){width=40%}

Now we can work out how to measure the space occupied in the Cartesian system by using the radius of the outer circle. We will obtain the following results:

$$size_1 = 2 \cdot radius$$

$$size_2 = \sqrt{3} \cdot radius$$

![The size of an hexagon, calculated](./images/developing_mechanics/hex_sizes.svg){width=40%}

With this, we can calculate the distances between the centres of the outer circles, in a way that allows us to create our hexmap.


$$dist_1 = \frac{3}{4} \cdot size_1 = \frac{3}{4} \cdot 2 \cdot radius = \frac{3}{2} \cdot radius$$

$$dist_2 = size_2 = \sqrt{3} \cdot radius$$

![Making a hexmap](./images/developing_mechanics/hexmap_done.svg){width=50%}

:::: note ::::
If you want to turn the hexagons "pointy side up", you just need to switch over the formulas.
::::::::::::::

{{placeholder}}
<!-- TODO: Quick talk about hexagonal tilemaps -->

#### Isometric Tilemaps

In some cases you may want to try and give your game a more "premium" feel: isometric maps can help you in that. The game is technically 2D, but the way the tiles are designed makes it look like it's a 3D game!

![A simple isometric tiles and a tilemap](./images/developing_mechanics/isometric_tiles.svg){width=60%}

Isometric tilemaps make use of more difficult algorithms, considering the odd shape the tiles are (usually they're diamond-shaped).

:::: trivia ::::
Diablo 2 is actually a 2D game that uses isometric tiles. Every item and character is a pre-rendered sprite: this means that every item drop is also pre-rendered and stored with some defined degrees of rotation to give a more variegated feeling.
::::::::::::::::

{{placeholder}}
<!-- TODO: Quick talk about isometric tilemaps -->

### Scrolling Backgrounds and Parallax Scrolling

#### Infinitely Scrolling Backgrounds {#infiniback}

When doing any kind of game that features a scrolling background, you should construct your art accordingly, allowing for enough variety to make the game interesting while avoiding creating huge artwork that weighs on the game's performance.

In a game that uses a scrolling background, the background used should be at least two times the screen size, in the scrolling direction ([Virtual Resolution] can prove really useful in this case) and the image should have what are called "loop points".

Loop points are points where the image repeats itself, thus allowing us to create an image that is virtually infinite, scrolling through the screen. To have a so-called "loop point" the image should be at least twice the size of the screen, in the scrolling direction.

The image below shows a background and its loop points.

![Demonstration of an image with loop points](./images/developing_mechanics/Loop_Points.png){width=40%}

To make an image appear like it's scrolling infinitely we need to move it back and forth between loop points when the screen passes over them.

For ease of explanation let's consider a screen scrolls towards the right, when we have reached a loop point, we reset the image position back to the position it was at the beginning and, since the image has been crafted to loop, the player won't notice that the background has been reset.

```{src='developing_mechanics/infiniscroll' caption='Example of an infinitely scrolling background implementation'}
```

#### Parallax Scrolling

Parallax in games is an effect that can give more depth to our environment: it looks like objects farther away are moving much slower than the objects closer to us.

This can be used to our advantage, along with some other tricks to enhance the perception of depth explained in [the chapter dedicated to graphics](#GraphicsResources).

Creating a parallax effect is quite easy: first we need at least two background layers (although three seems to be the best compromise between performance and depth of the effect):

- The **sprite layer** that will represent the closest layer to us, that will move at a certain speed that will be decided while developing the game;
- A **moving background** that will move slower compared to the sprite layer, thus giving the parallax effect;
- A **fixed background** that will represent our horizon and the farthest objects.

For the sake of clarity, we will re-use an image presented earlier to explain the "painter's algorithm":

![How we can split our game into layers](./images/gameloop/painter_algorithm.png){width=50%}

As stated earlier, a third optional background can be added to deepen the parallax scrolling effect, such background can take any of these positions:

- **Above** the sprite layer: in this case this "foreground layer" will need to move **faster** than the sprite layer and it should include very unobtrusive graphics, to avoid hiding important gameplay elements (like enemies);
- **Between the sprite layer and the first moving background**: in this case, the optional background should move **slower** than the sprite layer, but **faster** than the first moving background;
- **Between the first moving background and the fixed background**: in this case, the optional background will have to move **slower** than the first moving background.

The backgrounds should move all in the same direction, depending on the direction our character is moving: if our character is moving right, our moving backgrounds should move left.

Here's an example of how a simple parallax scrolling can be implemented between two rectangles:

```{src='developing_mechanics/parallax' caption='Example of parallax scrolling implementation'}
```

### Avoid interactions between different input systems

This is a small improvements that can be done on menu systems: if the player is using a keyboard to navigate the menu, the mouse should not interact with the menu.

In many frameworks when a fullscreen game window "captures" the mouse cursor, this is put on the center of the screen, which could be where a menu item is positioned.

Now imagine you are "flowing through" the menu, trying to load a saved file and the cursor is detected pointing at the "delete save file" option; you are briskly walking through the menu and what you think is the "do you want to load this file?" dialog is actually asking "do you want to **delete** this save file?". You select "yes" by pressing enter on your keyboard and your save file is gone!

This is an extreme edge case, but it could happen. Even if it is a minor annoyance like starting a new save file when instead you want to load an existing one, it diminishes the quality of your experience.

### Sprite Stenciling/Masking

{{placeholder}}

<!-- TODO: A shape that cuts out a sprite, pixel by pixel, allowing for certain effects like "going through a door" -->

### Loading screens {#loadingscreen}

If you load your resources in the same thread that executes the main game loop, your game will lock up while loading, which may trigger windows to ask you if you want to terminate the task. In this case it is better to dip our toes into [multi-threading](#multithreading) and create a proper loading screen.

The loading screen will be composed of two main components:

- **The graphical loading screen:** that will show the progress of the resource loading to the user, as well as tips and animations;
- **The actual resource loading thread:** that will take care to load the resources to the right containers, as well as communicating the global loading status to the loading screen in the main game loop.

We can represent the two "loops" in the following UML diagram:

![Rough UML diagram of a multi-threaded loading screen](./images/developing_mechanics/loading_multithread.svg){width=50%}

{{placeholder}}

<!-- TODO: Introduction on how to make multi-threaded loading screens -->

### Simulating Inertia

After learning how to move something on a screen, the next step is making the movement less "jarring" by introducing inertia. Before throwing solutions around, let's see what the problem is.

When we press a button, without inertia, our character starts moving at full speed towards the direction we defined, and it will stop as soon as we let go of the button. We can represent such behaviour with the following chart:

![Example chart of how movement without inertia looks](./images/developing_mechanics/inertia_0.svg){width=50%}

This could be jarring on its own, but the situation gets more serious the higher the speed difference: one thing is having a character being able to run from a standstill and stop immediately, but it feels all the more jarring if a character can turn 180 degrees on its path without the slightest hint of inertia (we assume that positive speed means "going right", while negative speed means "going left"):

![Example chart of how movement without inertia looks: reversing directions](./images/developing_mechanics/inertia_1.svg){width=50%}

If this is connected to a [fully-tracking camera system](#fully_tracking_camera) your player is in for a ride rivaling the deadliest rollercoasters in the world. What we want is the speed curve to behave more like the following (here too we assume that positive speed means "going right", while negative speed means "going left"):

![Example chart of how movement with inertia looks](./images/developing_mechanics/inertia_2.svg){width=50%}

A "softer" transition between directions can be a good way to avoid nausea as well as making the game behave more realistically, the change of direction can be also coupled with a skidding animation to make it even more convincing.

::: trivia :::
Inertia is so important (and common) that even the famous "Super Mario Bros." (1983) for the NES features it, as well as a "skidding animation".
::::::::::::::

In this section we will look at how to simulate inertia in a 1-dimensional space, where we can only move left or right.

When simulating inertia, the first things we need to know are:

- The top speed
- The acceleration rate
- The deceleration rate
- The direction we're going
- The direction we want to go

Let's think of the following situation, our character is running rightwards at a velocity `v`, measured in pixels per frame:

![Example of character running](./images/developing_mechanics/inertia_3.svg){width=50%}

This means that the character's `x` coordinate is moving every frame using the formula:

$$x_{n+1} = x_n + v$$

Now we suddenly want the player to start walking leftwards: we need to apply an acceleration `a` in that direction:

![Applying an acceleration to a character running](./images/developing_mechanics/inertia_4.svg){width=50%}

The new acceleration will influence the velocity, frame by frame, with the formula

$$v_{n+1} = v_n + a$$

Since velocity and acceleration have opposite directions, the acceleration we're applying will start "eating away the velocity" frame by frame, until our character starts moving leftwards. This "eating away" phase is what gives the feeling of inertia.

![Applying an acceleration frame by frame leads to the feeling of inertia](./images/developing_mechanics/inertia_5.svg){width=50%}

Being acceleration and velocity both vectors, we can apply an acceleration both in a one-dimensional way (like a 2D platformer) or a 2-dimensional way (like a space shooter) and the formulas will still be valid.

Deceleration is a special case of what we've seen so far, with the exception that the acceleration will always have direction opposite to velocity and as soon as velocity reaches zero, we stop applying it.

Now we can start writing some code:

```{src='developing_mechanics/inertia' caption='Code for simulating inertia'}
```

### Corner correction

Let's consider the following situation: our character is jumping, but due to the player being a bit too eager on their jump, the collision boxes are still slightly overlapping:

![What would be a good collision response for this situation?](./images/developing_mechanics/corner_correction_1.svg){width=50%}

The character's head is just slightly hitting the corner of the geometry, but since collision detection doesn't really "care" about the kind of movement you're doing the jump will be stopped.

Wouldn't it be better if instead the character was just "slightly pushed" to the right so to complete the jump?

![Corner correction makes for a more fluid experience](./images/developing_mechanics/corner_correction_2.svg){width=50%}

That would make it so the player doesn't get frustrated at a way-too-precise collision detection (remember me saying "you don't **want** precise collision detection?) and the game flow would be a lot smoother.

:::: tip ::::
This doesn't help only with side-scroller style run'n'jump games: if you're making a top-down game (like a 2D RPG) using tiles where movement is not tile-based. This will give you a smoother gameplay.
:::::::::::::

A possible implementation of a corner-correction algorithm, specifically for avoiding the jumping problem, could be the following:

```{src='developing_mechanics/corner_correction' caption='Possible implementation of a simple corner correction'}
```
