\null\clearpage

Tips, Tricks and Algorithms
==========================================

General Purpose
---------------

### Make that last Health Point count

Players love that rush of adrenaline they get when they escape a difficult situation with just one health point. That "just barely survived" situation can be "helped" by the game itself: some programmers decide to program the last HP in a special way.

Some prefer giving the last health point a value that is higher than the other health points (kind of like a "hidden health reserve"), others instead prefer giving a brief period of invincibility when that last "1HP" threshold is hit.

These small devices allow you to give players more of those "near death" experiences that can give players that confidence boost to keep them playing through a hard stage, while at the same time, reducing the chance that they will rage-quit.

### Telegraphing

Players hate the feeling of injustice that pops out when a boss pulls out a surprise attack, that's why in many games where precise defense movement is required bosses give out signals on the nature of their attack.

This "telegraphing" technique, allows for that "impending danger" feel, while still giving the player the opportunity to take action to counteract such attack.

Telegraphing is a nice way to suggest the player how to avoid screen-filling attacks (which would give the highest amount of "impending danger").

![Example of a telegraphed screen-filling attack in a shooter](./images/tips_tricks/telegraphing.png){width=60%}

### Scrolling Backgrounds and Parallax Scrolling

#### Infinitely Scrolling Backgrounds

When doing any kind of game that features a scrolling background, you should construct your art accordingly, allowing for enough variety to make the game interesting while avoiding creating huge artwork that weighs on the game's performance.

In a game that uses a scrolling background, the background used should be at least two times the screen size, in the scrolling direction ([Virtual Resolution] can prove really useful in this case) and the image should have what are called "loop points".

Loop points are points where the image repeats itself, thus allowing us to create an image that is virtually infinite, scrolling through the screen. To have a so-called "loop point" the image should be at least twice the size of the screen, in the scrolling direction.

The image below shows a background and its loop points.

![Demonstration of an image with loop points](./images/tips_tricks/Loop_Points.png){width=40%}

To make an image appear like it's scrolling infinitely we need to move it back and forth between loop points when the screen passes over them.

For ease of explanation let's consider a screen scrolls towards the right, when we have reached a loop point, we reset the image position back to the position it was at the beginning and, since the image has been crafted to loop, the player won't notice that the background has been reset.

\placeholder

#### Parallax Scrolling

Parallax in games is an effect that can give more depth to our environment: it looks like objects farther away are moving much slower than the objects closer to us.

This can be used to our advantage, along with some other tricks to enhance the perception of depth explained in [the chapter dedicated to graphics](#GraphicsResources).

Creating a parallax effect is quite easy: first we need at least two background layers (although three seems to be the best compromise between performance and depth of the effect):

- The **sprite layer** that will represent the closest layer to us, that will move at a certain speed that will be decided while developing the game;
- A **moving background** that will move slower compared to the sprite layer, thus giving the parallax effect;
- A **fixed background** that will represent our horizon and the farthest objects.

As stated earlier, a third optional background can be added to deepen the parallax scrolling effect, such background can take any of these positions:

- **Above** the sprite layer: in this case this "foreground layer" will need to move **faster** than the sprite layer and it should include very unobtrusive graphics, to avoid hiding important gameplay elements (like enemies);
- **Between the sprite layer and the first moving background**: in this case, the optional background should move **slower** than the sprite layer, but **faster** than the first moving background;
- **Between the first moving background and the fixed background**: in this case, the optional background will have to move **slower** than the first moving background.

The backgrounds should move all in the same direction, depending on the direction our character is moving: if our character is moving right, our moving backgrounds should move left.

### Sprite Stenciling/Masking

\placeholder

<!-- TODO: A shape that cuts out a sprite, pixel by pixel, allowing for certain effects like "going through a door" -->

2D Platformers
---------------

### Simulating Gravity

Gravity in a 2D platformer is quite easy to simulate: you just need to apply a constant acceleration towards the direction your gravity is pulling (it doesn't have to be towards the bottom of the screen!) and move your objects accordingly to such acceleration.

Your acceleration should not be precise (like the physics constant $9.81 m/s^2$), you don't want to make a physics engine: you want to make a somewhat convincing (or even better: entertaining) approximation of reality.

This is usually done before the player movement is used to update the character's status (but after the player input has been captured). Remember to add this acceleration before the collision detection is processed.

A useful precaution to avoid the [bullet through paper](#bulletthroughpaper) problem when you are working with long falls: put a limit at the fall velocity (kind of like air friction limits an object's fall velocity) of your objects. By applying a hard limit to the velocity, your gravity will be realistic but won't break your simulation.

\code{tips_tricks/gravity}{Code for applying gravity to an object}

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

A nice trick used mostly in 2D platformers to allow for smoother gameplay is "jump buffering".

Normally when a character is mid-air, the jump button does nothing, in code:

\code{tips_tricks/jump_buffering_nobuffer}{Code for jumping without buffering}

Jump Buffering consists in allowing the player to "buffer" a jump slightly before the character lands, making the controls a bit less stiff and the gameplay more fluid.

![Example of how jump buffering would work](./images/tips_tricks/jump_buffering.png){width=60%}

Jump buffering usually is put into practice using a timer, in a fashion similar to the following:

\code{tips_tricks/jump_buffering_buffer}{Jump buffering example}


### Coyote Time

Coyote time is a technique used to allow a player to jump a few frames after they fall off a platform, allowing for a more fluid gameplay.

![Example of how coyote time would work](./images/tips_tricks/coyote_time.png){width=60%}

The trick is starting a countdown as soon as the player leaves a platform without jumping, then if the player presses the jump button while that time is still going, they will perform the jump action, like they still were on a platform.

\code{tips_tricks/coyote_time}{Coyote time code example}

### Timed Jumps

A way to extend the mobility and challenge of a 2D platformer game is allowing players to jump higher the more the jump button is pressed: this allows the character to perform low and high jumps without much effort, making timing the jump button press a variable that adds to the challenge of a game.

![Example of how timed jumps would work](./images/tips_tricks/timed_jumps.png){width=60%}

To work well, timed jumps need to be implemented by tracking the jump button's `onPress` and `onRelease` events. When the jump button has just been pressed, the character's `Y` velocity will be set, as soon as the button is released, such velocity will be capped, shortening the jump height.

\code{tips_tricks/timed_jumps}{Example code of how timed jumps work}

Top-view RPG-Like Games
-----------------------

### Managing height

\placeholder

<!-- TODO: How to manage the different "heights" in the game -->

Rhythm Games
------------

### Synchronizing with the Music

\placeholder

<!-- TODO: Basic Beat detection or how to somehow sync music and gameplay -->

Match-x Games
--------------

### Managing and drawing the grid

When it comes to a match-x game, a good data structure for the play field is a matrix.

In most programming languages, a matrix is saved as an "array of arrays", where you have each element of an array representing a row, and each element of a row is a tile.

![Example of a matrix, saved as "array of arrays"](./images/tips_tricks/array_arrays.pdf){width=60%}

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

\code{tips_tricks/match3_findhorizontalmatches}{Finding horizontal matches in a match-3 game}

Let's talk a second about the last rows in the algorithm: they are specifically tailored to address a corner case that happens when there is a match that ends on the right border of the screen.

If such code was not there, the match number would grow by one, then the for loop would reset everything and we'd lose such match.

Similarly, we can make an algorithm that allows for vertical matches to be memorized for later removal:

\code{tips_tricks/match3_findverticalmatches}{Finding vertical matches in a match-3 game}

Both algorithms run in $O(n)$, where "n" is the number of tiles on the screen.

Now we can proceed to remove every tile that has been memorized as "part of a match", the quickest way may be to set such tile to "null" (or an equivalent value for your programming language).

#### Why don't we delete the matches immediately?

We could, but that would open the door to a pitfall that could be tough to manage: in case of a "T" match, we would find that the "horizontal matches" algorithm deletes part of said match, and the "vertical matches" algorithm wouldn't be able to complete the "T match", because the necessary tiles are deleted.

Let's see the image to understand better:

![What happens when deleting a match immediately](./images/tips_tricks/immediate_deletion.pdf){width=60%}

As visible from the first image, there is a T-shaped match involving cells 0,1,2 of row 0, cell 1 of row 1 and cell 1 of row 2.

If we deleted the horizontal match immediately, we would lose the possibility of completing the vertical match (highlighted in the second image).

Instead we memorize everything first, and then delete all the matches at once, without the risk of losing anything.

### Replacing the removed tiles and applying gravity

At this point, it is easy to make the "floating tiles" get into the right position: the hardest part is taking care of the graphics inbetweening that will give us that "falling effect" that we see in many match-x games.

After the graphics tweening, we need to create the new tiles that will go and fill up the holes that have been created by our matches and moved tiles.

After creating the tiles and tweening them in place, it will be necessary to check for more matches that have been created from the falling tiles (and eventually notify some kind of "combo system" to apply a "score multipier system" or even an achievement system using the [Observer Pattern](#ObserverPattern)).

\placeholder

<!-- TODO: Make the tiles fall by instantly setting the tile IDs in the matrix but tween the graphics -->
