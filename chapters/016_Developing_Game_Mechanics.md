\null\clearpage

Developing Game Mechanics
==========================

\epigraph{Fools say that they learn by experience. I prefer to profit by others' experience.}{\textit{Otto von Bismarck}}

In this section we will take a look at how to develop some known game mechanics, with pieces of code to aid you.

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

\code{developing_mechanics/iframes}{Example of I-Frames Implementation}

### Scrolling Backgrounds and Parallax Scrolling

#### Infinitely Scrolling Backgrounds {#infiniback}

When doing any kind of game that features a scrolling background, you should construct your art accordingly, allowing for enough variety to make the game interesting while avoiding creating huge artwork that weighs on the game's performance.

In a game that uses a scrolling background, the background used should be at least two times the screen size, in the scrolling direction ([Virtual Resolution] can prove really useful in this case) and the image should have what are called "loop points".

Loop points are points where the image repeats itself, thus allowing us to create an image that is virtually infinite, scrolling through the screen. To have a so-called "loop point" the image should be at least twice the size of the screen, in the scrolling direction.

The image below shows a background and its loop points.

![Demonstration of an image with loop points](./images/developing_mechanics/Loop_Points.png){width=40%}

To make an image appear like it's scrolling infinitely we need to move it back and forth between loop points when the screen passes over them.

For ease of explanation let's consider a screen scrolls towards the right, when we have reached a loop point, we reset the image position back to the position it was at the beginning and, since the image has been crafted to loop, the player won't notice that the background has been reset.

\code{developing_mechanics/infiniscroll}{Example of an infinitely scrolling background implementation}

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

### Avoid interactions between different input systems

This is a small improvements that can be done on menu systems: if the player is using a keyboard to navigate the menu, the mouse should not interact with the menu.

In many frameworks when a fullscreen game window "captures" the mouse cursor, this is put on the center of the screen, which could be where a menu item is positioned.

Now imagine you are "flowing through" the menu, trying to load a saved file and the cursor is detected pointing at the "delete savefile" option; you are briskly walking through the menu and what you think is the "do you want to load this file?" dialog is actually asking "do you want to **delete** this savefile?". You click "yes" and your savefile is gone!

This is an extreme edge case, but it could happen. Even if it is a minor annoyance like starting a new savefile when instead you want to load an existing one, it diminishes the quality of your experience.

### Sprite Stenciling/Masking

\placeholder

<!-- TODO: A shape that cuts out a sprite, pixel by pixel, allowing for certain effects like "going through a door" -->

### Loading screens {#loadingscreen}

If you load your resources in the same thread that executes the main game loop, your game will lock up while loading, which may trigger windows to ask you if you want to terminate the task. In this case it is better to dip our toes into [multithreading](#multithreading) and create a proper loading screen.

The loading screen will be composed of two main components:

- **The graphical loading screen:** that will show the progress of the resource loading to the user, as well as tips and animations;
- **The actual resource loading thread:** that will take care to load the resources to the right containers, as well as communicating the global loading status to the loading screen in the main game loop.

We can represent the two "loops" in the following UML diagram:

![Rough UML diagram of a multithreaded loading screen](./images/developing_mechanics/loading_multithread.pdf){width=50%}

\placeholder

<!-- TODO: Introduction on how to make multi-threaded loading screens -->

2D Platformers
---------------

### Simulating Gravity

Gravity in a 2D platformer is quite easy to simulate: you just need to apply a constant acceleration towards the direction your gravity is pulling (it doesn't have to be towards the bottom of the screen!) and move your objects accordingly to such acceleration.

Your acceleration should not be precise (like the physics constant $9.81 m/s^2$), you don't want to make a physics engine: you want to make a somewhat convincing (or even better: entertaining) approximation of reality.

This is usually done before the player movement is used to update the character's status (but after the player input has been captured). Remember to add this acceleration before the collision detection is processed.

A useful precaution to avoid the [bullet through paper](#bulletthroughpaper) problem when you are working with long falls: put a limit at the fall velocity (kind of like air friction limits an object's fall velocity) of your objects. By applying a hard limit to the velocity, your gravity will be realistic but won't break your simulation.

\code{developing_mechanics/gravity}{Code for applying gravity to an object}

### Avoiding "Floaty Jumps"

The previous trick shows a physics-accurate jumping: if we plot the height against time, we would get something that represents the curve of jump like the following:

![Plotting a physics-accurate jump](./images/developing_mechanics/physics_accurate_jump_plot.png){width=50%}

Although this can give the sensation that the character we're controlling is "floaty", which is not fun. In this case it's a better idea to enhance gravity when falling, to give the character some more "weight", which would be represented, more or less, by the following curve:

![Plotting a jump with enhanced gravity](./images/developing_mechanics/enhanced_gravity_jump_plot.png){width=50%}

This can be obtained with few simple lines of code, not very different from the gravity example of earlier:

\code{developing_mechanics/enhanced_gravity_jump}{Code for jump with enhanced gravity while falling}

In this example we are assuming that the framework used uses the screen coordinate system, and jumping brings the player from bottom towards the top of the screen. If you want different behaviour (like gravity inversion in puzzle games), something a tiny bit more involved may be in order.

### Ladders

\placeholder

<!-- TODO: How to allow the player to use ladders -->

### Walking on slanted ground

\placeholder

<!-- TODO: How to walk on slanted ground -->

### Stairs

\placeholder

<!-- TODO: How to walk on stairs -->

### Jump Buffering

A nice trick used mostly in 2D platformers to allow for smoother gameplay is "jump buffering", also known as "input buffering".

Normally when a character is mid-air, the jump button does nothing, in code:

\code{developing_mechanics/jump_buffering_nobuffer}{Code for jumping without buffering}

Jump Buffering consists in allowing the player to "buffer" a jump slightly before the character lands, making the controls a bit less stiff and the gameplay more fluid.

![Example of how jump buffering would work](./images/developing_mechanics/jump_buffering.png){width=60%}

Jump buffering usually is put into practice using a timer, in a fashion similar to the following:

\code{developing_mechanics/jump_buffering_buffer}{Jump buffering example}

### Coyote Time

Coyote time (also known as "edge tolerance") is a technique used to allow a player to jump a few frames after they fall off a platform, allowing for a more fluid gameplay.

![Example of how coyote time would work](./images/developing_mechanics/coyote_time.png){width=60%}

The trick is starting a countdown as soon as the player leaves a platform without jumping, then if the player presses the jump button while that time is still going, they will perform the jump action, like they still were on a platform.

\code{developing_mechanics/coyote_time}{Coyote time code example}

### Timed Jumps

A way to extend the mobility and challenge of a 2D platformer game is allowing players to jump higher the more the jump button is pressed: this allows the character to perform low and high jumps without much effort, making timing the jump button press a variable that adds to the challenge of a game.

![Example of how timed jumps would work](./images/developing_mechanics/timed_jumps.png){width=60%}

To work well, timed jumps need to be implemented by tracking the jump button's `onPress` and `onRelease` events. When the jump button has just been pressed, the character's `Y` velocity will be set, as soon as the button is released, such velocity will be capped, shortening the jump height.

\code{developing_mechanics/timed_jumps}{Example code of how timed jumps work}

Top-view RPG-Like Games
-----------------------

### Managing height

\placeholder

<!-- TODO: How to manage the different "heights" in the game -->

Rhythm Games
------------

### The world of lag

Welcome to the world of rhythm games, as with all new experiences we shall start with... the final boss: Lag.

Lag will be one of the most problematic things you will have to account for: things are not as easy as you may imagine when it comes to implementing a rhythm game. Let's see how to account for it, and eventually how to limit its effect on the player experience.

#### Input Lag

First of all: the ever-present "input lag": there is a certain time window between the moment the use presses a button and the moment the game receives such input. In the middle we find electrons running at breakneck speed through our keyboard circuitry, through the cable, to the motherboard, then the CPU, input abstraction layers in our OS, and finally the input system in our game.

And we didn't reach the game update stage yet.

Also we are not even accounting for the reaction time (about one second) from when the player sees something on screen and when they react.

Input lag is something that we cannot avoid, but there are countermeasures, as we will see below.

#### Video Lag

![Reference image for video lag](./images/developing_mechanics/video_lag.pdf){width=50%}

As with the input lag, there is also a not-neglegible video lag. The game has to prepare the image, send it to the video card, the card has to render it, apply effects and then send it to the screen, where the liquid crystals (or whatever technology we will have in the future) will have the re-align to create the colored pixels on screen.

#### Audio Lag

![Reference image for audio lag](./images/developing_mechanics/audio_lag.pdf){width=50%}

When the audio doesn't exactly match with the video, we talk about "audio lag", this has to be accounted for if you want to have a good rhythm game. In that case, there is a need to compensate for the audio lag, by starting each sound effect (or music) earlier or later by a well-defined amount of milliseconds.

#### Lag Tests

When it comes to lag, it is really difficult to estimate how the computer will react to our game, so we need a metric that will tell us what corrections we need to apply.

Such corrections are estimated comparing video and audio to the input: this way we will keep everything syncronized to the player input, making the game feel tighter.

First kind of test is done "video vs. input", the player has to push a button when something on the screen happens (like pushing rhythmically with a dot changing color), this way we can account for the video lag, compared to the input. This means we will obtain a $(video+input)$ lag measurement.

The second test done is the "audio vs. input" one, the player has to push a button when a sound cue happens on their speakers/headphones (like pushing rhythmically with a beep), this way we can account for the audio lag, compared to the input. This way we will obtain a $(audio+input)$ lag measurement.

By simple math we can account for the "video vs. audio" lag, like follows:

$$ (video + input) - (audio + input)$$

$$ video + input - audio - input$$

$$ video + \bcancel{input} - audio - \bcancel{input}$$

$$ video - audio $$


\placeholder
<!-- TODO: Talk on how to use lag tests to account for video and audio lag, and eventually use some gamification to make them more fun to do -->

### Synchronizing with the Music

\placeholder

<!-- TODO: Basic Beat detection or how to somehow sync music and gameplay -->

#### Time domain vs. Frequency Domain

When we listen to music, we are essentially streaming a bunch of numbers as the time goes forward, so we can plot the amplitude of our waveform against time, as follows:

![Plotting amplitude against time](./images/developing_mechanics/time_domain.png){width=60%}

In this case, when the time is the "independent variable" that we use to base our work, it's said we're working in *time domain*.

When we are working with games, we don't really care about what will happen (music-wise) 5 minutes from now, instead we care about other things that are happening now. In that case, it may be interesting to work in *frequency domain*, which can look something like this:

![Plotting frequency domain](./images/developing_mechanics/frequency_domain.png){width=60%}

We can switch back and forth between the two domains with "transforms", the most used is the Fourier Transform, and one of the most used algorithms to do it on computer is "FFT" (Fast Fourier Transform).

#### The Fast Fourier Transform

\placeholder
<!-- TODO: FFT, to turn time domain into frequency domain -->

#### Beat Detection

\placeholder
<!-- TODO -->

Match-x Games
--------------

### Managing and drawing the grid

When it comes to a match-x game, a good data structure for the play field is a matrix.

In most programming languages, a matrix is saved as an "array of arrays", where you have each element of an array representing a row, and each element of a row is a tile.

![Example of a matrix, saved as "array of arrays"](./images/developing_mechanics/array_arrays.pdf){width=60%}

This is a really nice way to interpret a grid, but at the same time it can open the door to some pitfalls if you're not careful.

Many programming languages allow for direct access to an element inside an "array of arrays" by using multiple access operators (usually the `[]` operator) in a row.

Usually each element you access with the first `[]` operator represents the rows, while the second time you use `[]` you will access the columns, this will make it so you need to access an element directly as follows: `matrix[y][x]` where "y" is the row number and "x" is the column number, which can prove counter-intuitive.

In the previous example, if we want to access the highlighted item, at the third row (indexed at 2), and in the fifth column (indexed at 4). We have to use `matrix[2][4]`, which is the opposite of what many people are used to when they think in `(x,y)` coordinates.

Try to keep visual representation and data structures separated in your mind, to avoid confusion.

### Finding and removing Matches

If you're doing a simple match-x game where you can only match tiles horizontally or vertically, the algorithm to find matches is quite simple, both conceptually and computationally.

The main idea is dividing the "horizontal matches" from the "vertical" ones. This will allow to simplify the algorithm and avoid some pitfalls (unless you want to give bonuses for "T-shaped" and "L shaped" matches).

For horizontal matches the idea is running through each row, keeping some variables representing the length of the match, as well as the color of the current "ongoing" match. As soon as we find a different color, if the length of the "ongoing" match is higher than "x" (usually 3), we save the references to the tiles involved for later removal.

Similarly we can do the same algorithm for vertical matches, by running through each column and saving the matches.

Here is a pseudo-code example:

\code{developing_mechanics/match3_findhorizontalmatches}{Finding horizontal matches in a match-3 game}

Let's talk a second about the last rows in the algorithm: they are specifically tailored to address a corner case that happens when there is a match that ends on the right border of the screen.

If such code was not there, the match number would grow by one, then the for loop would reset everything and we'd lose such match.

Similarly, we can make an algorithm that allows for vertical matches to be memorized for later removal:

\code{developing_mechanics/match3_findverticalmatches}{Finding vertical matches in a match-3 game}

Both algorithms run in $O(n)$, where "n" is the number of tiles on the screen.

Now we can proceed to remove every tile that has been memorized as "part of a match", the quickest way may be to set such tile to "null" (or an equivalent value for your programming language).

#### Why don't we delete the matches immediately?

We could, but that would open the door to a pitfall that could be tough to manage: in case of a "T" match, we would find that the "horizontal matches" algorithm deletes part of said match, and the "vertical matches" algorithm wouldn't be able to complete the "T match", because the necessary tiles are deleted.

Let's see the image to understand better:

![What happens when deleting a match immediately](./images/developing_mechanics/immediate_deletion.pdf){width=60%}

As visible from the first image, there is a T-shaped match involving cells 0,1,2 of row 0, cell 1 of row 1 and cell 1 of row 2.

If we deleted the horizontal match immediately, we would lose the possibility of completing the vertical match (highlighted in the second image).

Instead we memorize everything first, and then delete all the matches at once, without the risk of losing anything.

### Replacing the removed tiles and applying gravity

At this point, it is easy to make the "floating tiles" get into the right position: the hardest part is taking care of the graphics inbetweening that will give us that "falling effect" that we see in many match-x games.

After the graphics tweening, we need to create the new tiles that will go and fill up the holes that have been created by our matches and moved tiles.

After creating the tiles and tweening them in place, it will be necessary to check for more matches that have been created from the falling tiles (and eventually notify some kind of "combo system" to apply a "score multipier system" or even an achievement system using the [Observer Pattern](#ObserverPattern)).

\placeholder

<!-- TODO: Make the tiles fall by instantly setting the tile IDs in the matrix but tween the graphics -->

Cutscenes
---------

When you want to advance your storyline, a great tool is surely the undervalued cutscene. The game temporarily limits its interactivity and becomes more "movie-like", making the storyline go forward. Cutscenes can be scripted or just true video files; in this chapter we will analyze the difference between the two, advantages and disadvantages of both and how to implement each one, from a high-level perspective.

### Videos

The easiest way to implement cutscenes in most engines and frameworks, is to use videos. Many frameworks have native support for reproducing multimedia files with just a few lines of code, which makes this the preferred choice when it comes to the code.

The bad thing is that videos are a "static" format. They have their own resolution, their own compression and characteristics, this means that when a video is presented at a higher resolution that is own native one, we're bound to have artifacts due to upscaling.

\placeholder

<!-- TODO -->

### Scripted Cutscenes

\placeholder

<!-- TODO -->
