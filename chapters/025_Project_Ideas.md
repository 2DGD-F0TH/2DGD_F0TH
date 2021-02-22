{{pagebreak}}

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

Space Invaders
--------------

Manufactured by Taito in 1978, the arcade Space Invaders is probably one of the most known games around for its historical value.

![Example of a "Space Invaders"-style game](./images/project_ideas/space_invaders.pdf){width=50%}

Your objective is to prevent a horde of aliens from landing, by shooting them with your monochrome laser cannon. The concept is deceptively simple, but the implementation can be really complex if you look more thoroughly. The more aliens are killed, the more the remaining ones descend faster, there is a bonus ship that pops out at random intervals, the aliens shoot back at you, there are destructible "shields" to give you some defense from the alien bullets, the win and lose mechanics...

::::: trivia :::::
The aliens getting faster wasn't initially intended in Space Invaders, the position of the aliens gets updated every frame, but the hardware couldn't process all the entities fast enough. The more aliens got killed, the less entities had to be processed per frame, making the aliens move faster.

Instead of coming up with a solution, this "bug" was kept as a challenging "feature"
::::::::::::::::::

### Basic Level

In the basic level, we will just create a static, unarmed horde of aliens menacing our base (although being static it won't be much of a threat), we can shoot out projectiles at them and they disappear when they're hit, adding some points to our score.

Since our unwanted alien guests are not much of a threat, we won't need to put up any shields to defend against bullets. When all the aliens are dead, we win the game.

Skills Required:

- Drawing on screen;
- Collision Detection;
- Keyboard Input;
- Bullets;
- Score management;
- Winning conditions;
- Managing entities.

### Advanced Level

For the advanced level, we will make our alien foes more menacing by introducing movement and making the shoot at random intervals towards out laser cannon. The aliens do not need to accelerate as in the original game (but it's not really difficult to implement).

When an alien touches the ground, we will lose the game. Thus making the game a bit more difficult to manage (as well as to play).

We will also introduce a bonus ship that appears on top of the alien horde, floating from left to right, awarding a bonus when successfully hit.

::::: tip :::::
To make the bonus ship, you may be tempted to create an entirely new object, but at the same time a "bonus ship" IS-AN "alien ship", which would call for subclassing.

If you separate behaviour from the objects well, with methods, you can use subclassing to "change the ship's behaviour" by overriding the `update()` method.
:::::::::::::::

Further Skills Required:

- Timers (for the ship movement, and if you want, the bonus ship);
- Random number generation (for the bonus ship and the alien bullets);
- Subclassing;
- Losing conditions.

### Master level

For the master level we will make the shields, which are complex to code in the way of the original game, so we will "cheat".

Our shields will be "force fields" which can withstand 3 shots before getting disabled: such shots can come from the aliens or our ship. After 5 to 10 seconds, the shields will come back online and ready to stop bullet again.

Like the original game, the shields can block both our and our enemy's bullets, making for a more challenging gameplay.

For an even more challenging gameplay, we can (rarely) make a random ship detach from the horde to try and attack us directly: on such event, the ship will start shooting more often and move towards our laser cannon, before going back into position (a bit like galaxian).

Further Skills Required:

- Timers (this time for the shields);
- Minimal AI (for the "ship attack");
- Managing object states.

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

Shooter Arena
-------------

![Possible Shooter Arena Game](./images/project_ideas/shooter_arena.png){width=50%}

This is a simple mouse-controlled arena shooting game, much in the style of "Crimsonland" and "R.I.P.": the objective of the game is surviving for as long as possible.

### Basic Level

This level entails making the basic game: being a horde-based game, the AI can be really basic, following your movements and trying to touch you.

A single type of monsters will spawn from outside the screen and slowly make its way towards you, while the playable character will shoot bullets towards the mouse cursor.

If a bullet hits an enemy it will die and your score will be incremented, if an enemy hits you you will die.

Skills required:

- Drawing to a screen;
- Vectors;
- Basic AI;
- Collision detection and reaction;
- Projectiles;
- Lose Conditions;
- Mouse/Keyboard Controls;
- Spawning entities;
- Score Keeping.

### Advanced level

In the advanced level you should implement a life system for our playable character, so instead of dying our player will get hurt (thus reducing its health) and will have its life reduced. A health bar should be shown on screen.

The same should be done for each enemy, so you will need to be able to manage the state of each object (enemy) separatedly. Extra points if you show a small healthbar on top of a hit enemy, showing its current health.

You should also implement a way to easily code in new enemy types, this will require refactoring your code to support importing entities from data. Each enemy should at least have different speeds and health.

On each death, the enemy should have a random chance of dropping e medkit that will heal you when touched, such item should stay on screen for a limited amount of time, then disappear if not used.

Further Skills Required:

- Drawing an HUD;
- Managing an object's state;
- Coding entities as data;
- Random number generation;
- Timers.

### Master Level

In the most difficult level, you should start coding powerups, like new weapon types, temporarily increased walking speed, higher damage projectiles, etc... Similarly to medkits, these powerups should disappear after a certain amount of time.

An interesting weapon to implement would be a "railgun", with bullets that can go through enemies, this will be easier when you use ray casting (and some tricks for drawing), if you didn't use it already.

You should also animate the characters, thus getting used to your favourite engine's animator nodes/classes: this will make the game feel more complete.

On each death, the enemy should leave a blood (or if you prefer, goo) splatter that will disappear after a few seconds: this will make the game more messy and in its own way fun.

Further Skills Required:

- Animators;
- Ray Casting.

<!--
Simple Platformer
-----------------

{{placeholder}}
TODO: A 2D platformer without scrolling (maybe scrolling can be advanced)

Random Dot Gobbler
------------------

{{placeholder}}
TODO: A procedurally generated pacman-style game, with generated mazes, ghost AI can be advanced or master

Random Dungeon Crawler
----------------------

{{placeholder}}
TODO: Procedurally generated dungeon crawler, with treasure, exit with a key
-->
