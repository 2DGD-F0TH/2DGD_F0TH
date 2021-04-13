{{pagebreak}}

Developing Game Mechanics
==========================

:::::: {.epigraph author="Otto von Bismarck"}
Fools say that they learn by experience. I prefer to profit by others' experience.
::::::

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

```{src='developing_mechanics/iframes' caption='Example of I-Frames Implementation'}
```

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

### Avoid interactions between different input systems

This is a small improvements that can be done on menu systems: if the player is using a keyboard to navigate the menu, the mouse should not interact with the menu.

In many frameworks when a fullscreen game window "captures" the mouse cursor, this is put on the center of the screen, which could be where a menu item is positioned.

Now imagine you are "flowing through" the menu, trying to load a saved file and the cursor is detected pointing at the "delete savefile" option; you are briskly walking through the menu and what you think is the "do you want to load this file?" dialog is actually asking "do you want to **delete** this savefile?". You click "yes" and your savefile is gone!

This is an extreme edge case, but it could happen. Even if it is a minor annoyance like starting a new savefile when instead you want to load an existing one, it diminishes the quality of your experience.

### Sprite Stenciling/Masking

{{placeholder}}

<!-- TODO: A shape that cuts out a sprite, pixel by pixel, allowing for certain effects like "going through a door" -->

### Loading screens {#loadingscreen}

If you load your resources in the same thread that executes the main game loop, your game will lock up while loading, which may trigger windows to ask you if you want to terminate the task. In this case it is better to dip our toes into [multithreading](#multithreading) and create a proper loading screen.

The loading screen will be composed of two main components:

- **The graphical loading screen:** that will show the progress of the resource loading to the user, as well as tips and animations;
- **The actual resource loading thread:** that will take care to load the resources to the right containers, as well as communicating the global loading status to the loading screen in the main game loop.

We can represent the two "loops" in the following UML diagram:

![Rough UML diagram of a multithreaded loading screen](./images/developing_mechanics/loading_multithread.svg){width=50%}

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

{{placeholder}}

<!-- TODO: How to simulate starting and finishing a run from and to a standstill, without being too jarring (NEEDS CODE) -->

2D Platformers
---------------

### Simulating Gravity

Gravity in a 2D platformer is quite easy to simulate: you just need to apply a constant acceleration towards the direction your gravity is pulling (it doesn't have to be towards the bottom of the screen!) and move your objects accordingly to such acceleration.

Your acceleration should not be precise (like the physics constant $9.81 m/s^2$), you don't want to make a physics engine: you want to make a somewhat convincing (or even better: entertaining) approximation of reality.

This is usually done before the player movement is used to update the character's status (but after the player input has been captured). Remember to add this acceleration before the collision detection is processed.

A useful precaution to avoid the [bullet through paper](#bulletthroughpaper) problem when you are working with long falls: put a limit at the fall velocity (kind of like air friction limits an object's fall velocity) of your objects. By applying a hard limit to the velocity, your gravity will be realistic but won't break your simulation.

```{src='developing_mechanics/gravity' caption='Code for applying gravity to an object'}
```

### Avoiding "Floaty Jumps"

The previous trick shows a physics-accurate jumping: if we plot the height against time, we would get something that represents the curve of jump like the following:

![Plotting a physics-accurate jump](./images/developing_mechanics/physics_accurate_jump_plot.png){width=50%}

Although this can give the sensation that the character we're controlling is "floaty", which is not fun. In this case it's a better idea to enhance gravity when falling, to give the character some more "weight", which would be represented, more or less, by the following curve:

![Plotting a jump with enhanced gravity](./images/developing_mechanics/enhanced_gravity_jump_plot.png){width=50%}

This can be obtained with few simple lines of code, not very different from the gravity example of earlier:

```{src='developing_mechanics/enhanced_gravity_jump' caption='Code for jump with enhanced gravity while falling'}
```

In this example we are assuming that the framework used uses the screen coordinate system, and jumping brings the player from bottom towards the top of the screen. If you want different behaviour (like gravity inversion in puzzle games), something a tiny bit more involved may be in order.

### Ladders

{{placeholder}}

<!-- TODO: How to allow the player to use ladders -->

### Walking on slanted ground

{{placeholder}}

<!-- TODO: How to walk on slanted ground -->

### Stairs

{{placeholder}}

<!-- TODO: How to walk on stairs -->

### Jump Buffering {#jump_buffering}

A nice trick used mostly in 2D platformers to allow for smoother gameplay is "jump buffering", also known as "input buffering".

Normally when a character is mid-air, the jump button does nothing, in code:

```{src='developing_mechanics/jump_buffering_nobuffer' caption='Code for jumping without buffering'}
```

Jump Buffering consists in allowing the player to "buffer" a jump slightly before the character lands, making the controls a bit less stiff and the gameplay more fluid.

![Example of how jump buffering would work](./images/developing_mechanics/jump_buffering.png){width=60%}

Jump buffering usually is put into practice using a timer, in a fashion similar to the following:

```{src='developing_mechanics/jump_buffering_buffer' caption='Jump buffering example'}
```

### Coyote Time {#coyote_time}

Coyote time (also known as "edge tolerance") is a technique used to allow a player to jump a few frames after they fall off a platform, allowing for a more fluid gameplay.

![Example of how coyote time would work](./images/developing_mechanics/coyote_time.png){width=60%}

The trick is starting a countdown as soon as the player leaves a platform without jumping, then if the player presses the jump button while that time is still going, they will perform the jump action, like they still were on a platform.

```{src='developing_mechanics/coyote_time' caption='Coyote time code example'}
```

### Timed Jumps

A way to extend the mobility and challenge of a 2D platformer game is allowing players to jump higher the more the jump button is pressed: this allows the character to perform low and high jumps without much effort, making timing the jump button press a variable that adds to the challenge of a game.

![Example of how timed jumps would work](./images/developing_mechanics/timed_jumps.png){width=60%}

To work well, timed jumps need to be implemented by tracking the jump button's `onPress` and `onRelease` events. When the jump button has just been pressed, the character's `Y` velocity will be set, as soon as the button is released, such velocity will be capped, shortening the jump height.

```{src='developing_mechanics/timed_jumps' caption='Example code of how timed jumps work'}
```

Top-view RPG-Like Games
-----------------------

### Managing height

{{placeholder}}

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

![Reference image for video lag](./images/developing_mechanics/video_lag.svg){width=50%}

As with the input lag, there is also a not-neglegible video lag. The game has to prepare the image, send it to the video card, the card has to render it, apply effects and then send it to the screen, where the liquid crystals (or whatever technology we will have in the future) will have the re-align to create the colored pixels on screen.

#### Audio Lag

![Reference image for audio lag](./images/developing_mechanics/audio_lag.svg){width=50%}

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


{{placeholder}}
<!-- TODO: Talk on how to use lag tests to account for video and audio lag, and eventually use some gamification to make them more fun to do -->

### Synchronizing with the Music

{{placeholder}}

<!-- TODO: Basic Beat detection or how to somehow sync music and gameplay -->

#### Time domain vs. Frequency Domain

When we listen to music, we are essentially streaming a bunch of numbers as the time goes forward, so we can plot the amplitude of our waveform against time, as follows:

![Plotting amplitude against time](./images/developing_mechanics/time_domain.png){width=60%}

In this case, when the time is the "independent variable" that we use to base our work, it's said we're working in *time domain*.

When we are working with games, we don't really care about what will happen (music-wise) 5 minutes from now, instead we care about other things that are happening now. In that case, it may be interesting to work in *frequency domain*, which can look something like this:

![Plotting frequency domain](./images/developing_mechanics/frequency_domain.png){width=60%}

We can switch back and forth between the two domains with "transforms", the most used is the Fourier Transform, and one of the most used algorithms to do it on computer is "FFT" (Fast Fourier Transform).

#### The Fast Fourier Transform

{{placeholder}}
<!-- TODO: FFT, to turn time domain into frequency domain -->

#### Beat Detection

{{placeholder}}
<!-- TODO: How to perform beat detection in games -->

"Bullet Hell" Style Games
-------------------------

The common definition of a "bullet hell" game is usually the one of a scrolling (usually space-themed) shooter with a very high level of difficulty and lots of enemy bullets on screen (hence the name).

### Bullets

When it comes to this kind of game, it is vital that the enemy bullets are **well visible** (as stated in the [shooters section](#gd_shooters) in the "game design" chapter), this usually means that their color is brighter and has a lot of contrast with the background and the sprites on screen.

![Example of how to better "highlight" bullets](./images/developing_mechanics/bh_highlight_bullets.png){width=50%}

Having "evident" enemy bullets makes the situation easier to assess, even when the situation becomes really chaotic. If you zoom out (or get your reading support farther from your eyes) you can see that the "non-highlighted" bullets (on the left side) tend to "blend in", while the "highlighted" (on the right side) version stay visible.

To highlight bullets, you can use "complementary colors", as shown in the [use contrast to your advantage](#contrast_to_your_advantage) section.

::: tip :::
Bullet visibility is so important that in many games bullets are the last thing to be drawn before the player: this means they're drawn over explosions, other enemies and your own bullets too.

If you let players lose sight of bullets by drawing graphical effects over them, the game will feel unfair.
:::::::::::


### The Ship Hitbox

In the bullet hell genre usually the player ship's (or character of some kind) hitbox is usually much smaller than the visible sprite, this makes the game a little bit "easier than it seems", but at the same time it doesn't mean that the game is easy either.

![An example of a Bullet Hell ship hitbox](./images/developing_mechanics/bh_ship_hitbox.png){width=50%}

In this image, the ship's hitbox is limited to the cockpit, some games prefer some area that could be considered the "ship engine" while others just have a "core" of some sort.

Many games of the genre even make the hitbox a single pixel!

### Screen-clearing bombs

Another mechanic used in bullet hell games are "screen-clearing bombs": these are used to rid the screen of the gigantic number of bullets on it, to give the player some breathing room.

In some games bombs may be also used to destroy small enemies and damage bigger ones. The screen clearing move can happen in many ways: the most common is just making the bullets disappear, but other games prefer turning the "destroyed bullets" into small collectibles that can give the player points.

### Clearing bullets on pattern changes

Some bullet hell games feature multi-phase bosses, where the boss changes attack strategy, and thus their bullet pattern and speed, at certain points of the fight (usually when reaching a certain amount of health left). This may create some issues to the player, since the new bullets may cover all "escape routes" willingly left by the previous bullets, thus making it impossible to not die.

A simple and effective strategy is clearing the screen of the enemy bullets automatically when the boss changes phase (sometimes transforming the bullets into collectibles for score), this will allow for a quick breather to the player, as well as a somewhat smooth transition to the new phase.

### Find other chances to clear some bullets

Some games find creative ways to clean up a screen cluttered with bullets: for instance some bullets can turn into collectibles when a pickup is touched by the ship.

::: trivia :::
In ZenoDyne R powerups are real, "physical" objects, and as such they block incoming bullets, so they can be strategically used as a "shield", and then pick them up at the last second.
::::::::::::::

Some games like to clear the screen (without giving out collectibles) at the beginning of a boss fight, to give a "clean slate" to start the boss with.

### Turn enemy bullets into collectibles at the end of a boss fight

An interesting form of bonus that is often present in bullet hell games is turning all the boss's bullets on screen into collectibles at the end of a boss fight.

Since this genre of game gets progressively harder the more bullets are on screen, this small trick rewards players for being good at dodging, while players who used screen-clearing bombs will have a smaller bonus.

### The "Chain Meter"

This is a mechanic used in many bullet hell games: the chain meter is a meter that gains value according to the number of enemies you kill in a certain amount of time and giving a score multiplier according to it.

This meter will automatically discharge with time, making it hard to keep up a high score multiplier, adding challenge to the player and rewarding them for being good at destroying enemies in large numbers fast.

Usually the meter has 5 levels, starting from level 1, when the meter is full, the meter "gains a level" and a score multiplier is applied accordingly. For instance we can have:

- Level 1: 1x multiplier (normal score)
- Level 2: 2x multiplier (double score for each killed enemy)
- ...
- Level 5: 5x multiplier

::: tip :::
You can code the "discharge" so it is faster at higher levels. This will bring even more challenge at keeping the level high.
:::::::::::

When the player dies, the counter gets completely emptied and thus the multiplier gets reset to 1x.

::: tip :::
Alternatively, you can halve the level of the meter on player's death.
:::::::::::

### Managing the player's death

It is very common in the "bullet hell" genre to punish the player's death with a strong cut at the ship's power.

This has a problem: a player dying may spiral into a fully-fledged game over because the ship is now extremely underpowered compared to the stage the player died in.

A solution often used in this genre of games is having a dying player's ship have a random chance of releasing a random number of powerups and bomb pickups on death, thus allowing the now-weakened player to "regain some strength" and continue their game.

### The Enemy AI

Probably the hardest part to develop in a "bullet hell" style game is the enemy AI and how to make the enemy bullets form a pattern that is hard but not impossible to dodge.

Since the gameplay is very hard to balance, this genre seldom sees a "procedural game" (an exception that comes to my mind is "Task force Kampas", which features procedural levels, but handmade bosses).

A way to make the game feel more fair is programming the AI so it doesn't shoot "on the way out" of the screen. Each enemy essentially has 3 phases in its AI:

1. Enter the screen
2. Fight (usually by shooting a single pattern or a continuous stream of the same pattern)
3. Exit the screen (or die)

When the enemy exits the screen, it should stop shooting and just orderly leave.

::: tip :::
If your game features enemy turrets, they should stop shooting when they are behind the player's ship: the player is already busy enough handling shots from the front. Shooting from behind makes the game unfair.
:::::::::::

### Be fair to the player, but also to the computer

The title may be a bit awkward: how can you "be fair to a computer"?

Computers don't have feelings, but players do. And letting the players kill the enemy before the AI activates takes away all the challenge from the game itself: the enemies become cannon fodder when the player's weapons have enough "power" to instantly kill most of the enemy forces.

So to apply this, you should probably make the enemies invincible until they're fully on screen: this way the player sees them and doesn't kill them beyond the top of the game area.

### Inertia

Control is everything in a game where a pixel can be the difference between life and death of your ship/character. This means that heavy inertia does not play well with the "bullet hell" genre.

This also doesn't mean that you can't apply any, just be careful and don't go overboard. When a player dies because their ship went too far due to inertia, they will get mad at the game, and by transitive property, at the devs.

### Some examples

There are games that make the most of the "bullet hell" mechanics to give player more challenge, or risk/reward choice.

One game is "Touhou", which has a "grazing" mechanic: if a bullet slightly grazes (but does not hit) your hitbox, you will see some sparks and get a bonus in points.

Another title that makes the most of giving the player a "risk vs reward" choice is Ikaruga, with it's "polarity" mechanic. Your ship has two sides: black and white, each side is able to absorb (and so is also immune) to the bullets of the same color, but also does more damage to the enemies of the opposite color.

Match-x Games
--------------

### Managing and drawing the grid

When it comes to a match-x game, a good data structure for the play field is a matrix.

In most programming languages, a matrix is saved as an "array of arrays", where you have each element of an array representing a row, and each element of a row is a tile.

![Example of a matrix, saved as "array of arrays"](./images/developing_mechanics/array_arrays.svg){width=60%}

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

```{src='developing_mechanics/match3_findhorizontalmatches' caption='Finding horizontal matches in a match-3 game'}
```

Let's talk a second about the last rows in the algorithm: they are specifically tailored to address a corner case that happens when there is a match that ends on the right border of the screen.

If such code was not there, the match number would grow by one, then the for loop would reset everything and we'd lose such match.

Similarly, we can make an algorithm that allows for vertical matches to be memorized for later removal:

```{src='developing_mechanics/match3_findverticalmatches' caption='Finding vertical matches in a match-3 game'}
```

Both algorithms run in $O(n)$, where "n" is the number of tiles on the screen.

Now we can proceed to remove every tile that has been memorized as "part of a match", the quickest way may be to set such tile to "null" (or an equivalent value for your programming language).

#### Why don't we delete the matches immediately?

We could, but that would open the door to a pitfall that could be tough to manage: in case of a "T" match, we would find that the "horizontal matches" algorithm deletes part of said match, and the "vertical matches" algorithm wouldn't be able to complete the "T match", because the necessary tiles are deleted.

Let's see the image to understand better:

![What happens when deleting a match immediately](./images/developing_mechanics/immediate_deletion.svg){width=60%}

As visible from the first image, there is a T-shaped match involving cells 0,1,2 of row 0, cell 1 of row 1 and cell 1 of row 2.

If we deleted the horizontal match immediately, we would lose the possibility of completing the vertical match (highlighted in the second image).

Instead we memorize everything first, and then delete all the matches at once, without the risk of losing anything.

### Replacing the removed tiles and applying gravity

At this point, it is easy to make the "floating tiles" get into the right position: the hardest part is taking care of the graphics inbetweening that will give us that "falling effect" that we see in many match-x games.

After the graphics tweening, we need to create the new tiles that will go and fill up the holes that have been created by our matches and moved tiles.

After creating the tiles and tweening them in place, it will be necessary to check for more matches that have been created from the falling tiles (and eventually notify some kind of "combo system" to apply a "score multipier system" or even an achievement system using the [Observer Pattern](#ObserverPattern)).

{{placeholder}}

<!-- TODO: Make the tiles fall by instantly setting the tile IDs in the matrix but tween the graphics -->

Cutscenes
---------

When you want to advance your storyline, a great tool is surely the undervalued cutscene. The game temporarily limits its interactivity and becomes more "movie-like", making the storyline go forward. Cutscenes can be scripted or just true video files; in this chapter we will analyze the difference between the two, advantages and disadvantages of both and how to implement each one, from a high-level perspective.

### Videos

The easiest way to implement cutscenes in most engines and frameworks, is to use videos. Many frameworks have native support for reproducing multimedia files with just a few lines of code, which makes this the preferred choice when it comes to the code.

The bad thing is that videos are a "static" format. They have their own resolution, their own compression and characteristics, this means that when a video is presented at a higher resolution that is own native one, we're bound to have artifacts due to upscaling.

{{placeholder}}

<!-- TODO: provide example code on how to add video cutscenes -->

### Scripted Cutscenes

{{placeholder}}

<!-- TODO: Talk about how to implement a "scripted cutscene" system -->
