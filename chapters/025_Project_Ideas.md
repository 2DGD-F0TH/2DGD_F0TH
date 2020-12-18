\null\clearpage

Project Ideas
=============

\epigraph{In theory, theory and practice are the same. In practice, they are not.}{\textit{Albert Einstein}}

This section tries to give you some ideas for some small projects you can try to do by yourself.

The projects will be put in order of (perceived) difficulty and each one will use a larger set of skills you have learned. Each project will have three levels of completion:

- **Basic:** This is the baseline to consider a project "complete", this is also the set of requirements the projects are sorted by;
- **Advanced:** This is more of a challenge, requiring more advanced skills and techniques. If you complete this level, you can consider yourself comfortable with most of the matters treated in the project;
- **Master:** This can be a real challenge, requiring skills and techniques that may not be taught in this book, but such problems can be solved with a bit of planning. If you complete this level successfully, pat yourself on the back and keep up the great work!

A title screen for each game is not required or necessary, but if you want to make one, feel free to do so.

Tic-Tac-Toe
-----------

![Example picture of Tic-Tac-Toe](./images/project_ideas/tic_tac_toe.png){width=50%}

Tic-Tac-Toe is a "finite" game, there are a finite number of choices and strategies but that makes it ideal for a simple project.

The objective of the game is to score 3 of your own symbol (either an `X` or an `O`) in a row or in diagonal, in a 3x3 grid; the players take turns and put their symbol on the game board in an empty spot.

### Basic Level

Make a simple Tic-Tac-Toe style of game, where the mouse commands both players (so it alternatively switches between `X` and `O`), the game should be able to detect winning conditions for a certain symbol or a draw.

Skills required:

- Drawing to a screen;
- Event handling;
- Mouse Events;
- Winning/Losing Conditions.

### Advanced Level

Make a computer-controlled player, by making the mouse write only the `X` symbol, while the computer will randomly put the `O` symbol onto a blank spot. The game should still be able to detect winning conditions for each player, as well as a draw.

Furthermore, the game should enforce turns, so that the human player is not able to put their own symbol when it is the computer's turn.

Further skills required:

- Random number generation;
- Coding basic game logic.

### Master Level

Improve your computer-controlled player by making it actually seek for a way to win against the human player. The AI should do these checks, in order:

1. "Can I make a 3-in-a-row in a single move?": this means that the AI has 2-in-a-row with an empty space available. The AI should put their symbol in the empty spot to win.
2. "Is my adversary about to make a 3-in-a-row?": this means the human player has 2-in-a-row with an empty space available. The AI reaction should be to put their symbol in the empty spot.
3. If none of those cases happen, then make the AI fall back to a "random choice".

The AI does not need to be perfect, just mildly challenging.

::::: tip :::::
To some people, points 1 and 2 may be problematic, in the case that two among rows, columns or diagonals satisfy one of those conditions.

If both satisfy condition 1, there is actually no problem, the AI would win anyway: the AI can choose randomly.

In case 2, the AI should check if there is a way to block both "potential wins" in one move, but it is a perfectly acceptable solution to consider the case a loss for the AI and just choose randomly.
:::::::::::::::

Breakout
--------

![Example picture of a breakout-style game](./images/project_ideas/breakout.png){width=50%}

Breakout is a well-known brick-breaker style of game, where the player drives a paddle trying to keep a ball from getting to the bottom of the screen, while breaking all the blocks on the screen.

### Basic Level

The basic level is just making the basic game: make a single level that works. If the player loses their ball, they lose, if they manage to clear the screen they win.

The paddle should be moved with mouse or keyboard (you can choose to implement either or both) with sufficient speed to avoid useless deaths but slow enough to be usable. The ball should bounce like it bounces on a wall: keeping the angle with the wall without changing it.

Skills Required:

- Drawing to a screen;
- Vectors;
- Moving elements of the game;
- Event handling;
- Winning/Lose Conditions;
- Mouse/Keyboard Controls;
- Collision detection and reaction;
- Object management (creation and deletion);
- Score keeping.

### Advanced Level

In the advanced level you should implement a basic life system: each time the ball crosses the lower side of the screen, you lose a life; when all lives are lost, you lose the game.

To make the game a bit more interesting, it could be an idea to implement different block types:

- **Explosive Blocks:** When destroyed, these blocks explode, destroying the surrounding blocks;
- **Multiple-hit Blocks:** They simply require more than one hit to destroy;
- **Score Blocks:** When destroyed, these blocks drop score items that slowly descend toward the bottom of the screen. If you catch these items with the paddle, you get extra points.

Furthermore, the ball should progressively get faster with gameplay, you can either do it every few bounces or just every bounce with the paddle. The choice is yours.

Further Skills Required:

- Managing Object's states;
- Subclassing;
- Managing Game State.

### Master Level

With the "master level" we are going to complete this game by adding powerups: they work in a similar way to the advanced level's score blocks with a descending item that grants a certain status to either the ball or the paddle.

You can make it a random chance for each block destruction or make a dedicated block. Here are some suggestions for some powerups:

- Larger/Smaller paddle;
- Paddle that shoots to destroy blocks;
- Faster/Slower ball;
- Ball that goes through blocks, destroying them;
- "Sticky Paddle" that allows to stop and then release the ball;
- Multiball.

Furthermore, you can implement a sort of "biased bouncing" for the paddle: the further left or right on the paddle the ball touches, the more it will take a horizontal bias towards that direction.

This way the center of the paddle is "neutral", keeping the normal bouncing mechanics, while the leftmost and rightmost sides allow the player to direct the ball the way they want.

![Biased Bouncing for breakout](./images/project_ideas/breakout_bias.png){width=50%}

Space Invaders
--------------

\placeholder
<!-- TODO: A space invaders style game -->

Shooter Arena
-------------

\placeholder
<!-- TODO: A crimsonland-style arena where enemies spawn from outside and invade the arena, you have to defend yourself with your weapon -->

Simple Platformer
-----------------

\placeholder
<!-- TODO: A 2D platformer without scrolling (maybe scrolling can be advanced) -->

Random Dot Gobbler
------------------

\placeholder
<!-- TODO: A procedurally generated pacman-style game, with generated mazes, ghost AI can be advanced or master -->

Random Dungeon Crawler
----------------------

\placeholder
<!-- TODO: Procedurally generated dungeon crawler, with treasure, exit with a key -->
