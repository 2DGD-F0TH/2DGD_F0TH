\null\clearpage
Balancing Your Game
===================

An imbalanced game is a frustrating game, and most of the time balancing a game is one of the toughest challenges a game developer/designer can find themselves to have to face.

Let's talk about some principles and guidelines that can help you balancing your game and keep your players challenged but not frustrated.

The "No BS" principle
----------------------

The "master principle" everyone should follow (in my humble opinion) is what I call the "no BS" principle.

You should not trade the "fun" of your game for any other mechanic (like showing an advertisement to allow them to continue playing), that is equivalent to betraying your player, makes the game feel unfair and un-fun.

Here are some examples of mechanics that break the "no BS" principle:

- **Sudden spikes in difficulty:** when you have a sudden spike in difficulty, the player feels stumped and the game tends to lose its charm, you are "interrupting the flow" of the game by placing an arbitrary hurdle on your players' road;
- **Off-screen instant-death traps:** having something deadly that pops out from off-screen and kills the player is unfair and will make your players scream "that's BS!" all the time, if you want to place some obstacles that pop from off-screen you should "telegraph" them. "Telegraphing" is a technique where you send a warning signal to the player that danger is coming. For instance a huge laser that instantly kills you should be preceded by a couple seconds by a yellow "!" signal on the right side of the screen, where the laser is due to strike. Another way to telegraph said laser would be to illuminate the part of the screen that is about to be stroke, like the light of the laser is coming up;
- **Arbitrary invisible time limits:** If you suddenly interrupt the player's game with a "time up" and you have no countdown on the screen, the player will get frustrated, that's for sure;
- **Taking control away from the player:** Not allowing the player to move (getting blocked by an enemy and killed) or just not allowing the player to adjust their jump mid-air is a surefire way to make them not play your game anymore.

Always favour the player
------------------------

In the process of balancing a game, as a game developer/designer you will surely find yourself in front of the following decision time and time again:

> Shall I favour the game's precision or should I give some leeway to the player?

The answer is always the latter.

Giving some leeway to the player, for instance by having a more generous hit-box that allows you to stay alive even if a bullet grazes your character makes the game seem more "fair".

There are infinite ways to make a game challenging without having to force the player into accepting very precise hit-boxes or extremely tight gameplay.

Cheating
---------

Cheating is the act of fraudulently altering the game's mechanics in advantage of the player, performed by the players themselves.

It is something that many game developers and designers have to battle against, so here are some suggestions and tips to limit cheating in your game.

### Single Player

Cheating in single player is an act that doesn't usually do a massive amount of damage, and such damage is usually confined inside the single "single-player" game.

Playing outside of the rules can be really fun (that's one of the principles the "glitch hunters" love: doing something outside of what another person imposed them), for instance some people cheat in games to bring some mayhem into their gameplay, or they use cheats implemented inside the game itself for a comedic factor (like the omnipresent "giant head" cheat).

Sometimes cheating happens because the game is unbalanced and breaks the "no BS" principle, an instance of this happening could be when a game has a great story and gameplay but there is a boss that is so hard the game just stops there. You want to see how the story continues, but the game has gone so much out of balance you are willing to break its own mechanics to be able to continue it.

In this case the approach you should have is rebalancing the game, instead of limiting your players.

When it comes to cheat prevention, usually the first order of action is giving the game the ability to "check the validity" of an instruction.

For instance if a player character has its coordinates at (5,5) on frame $n$ and coordinates at (1500, 5) at frame $n+1$, there is something fishing going on, since maybe the player can only move 500 pixels per second (while it moved 995 in one frame: $\frac{1}{60}$ of a second).

Such checks will slow down the processing, but will allow you to put a limit to cheating, possibly intervening in an active way, by resetting the space walked to the maximum amount possible in one frame, although this could give some issues with slower computers and [variable time steps](#variable_timesteps).

### Multiplayer

When it comes to multiplayer and "leaderboards", cheating can be create some major damage to the game's enjoyability. It is honestly disheartening seeing a level that has been completed in 0 seconds on top of the leaderboard, totally unreachable with normal gameplay.

When competitive gameplay comes into the picture, playing against a cheater is frustrating and maddening, you feel powerless and the game is not fun and sometimes it even feels "broken", even though it is stable and playable.

Here we will distinguish between the two main forms of multiplayer: Peer-to-peer gameplay and dedicated servers.

#### P2P

Peer-to-peer multiplayer is the economically cheapest and easiest way to implement multiplayer, two or more computers (or consoles) are on "the same level" and communicate directly with each other, without a tertiary server in the middle.

![Example of a P2P connection](./images/balancing/P2P.pdf){width=60%}

The main difficulty in preventing cheating is that there is no "authoritative game state", the program cannot know if either player is cheating besides having an array of "possible actions", like in single player, but with the added difficulty of network lag.

![Two cheaters meet in P2P](./images/balancing/cheat_P2P.pdf){width=60%}

Giving such "authoritative game state" to either of the players is not a good idea, because that way they would be able to cheat themselves and since they're the "game master", everything they do would be accepted.

![What would happen if one of the Peers had the autoritative game state](./images/balancing/auth_P2P.pdf){width=60%}

This is also the reason why many games that make use of P2P connections have implementations of anti-cheat systems that are shoddy at best.

#### Dedicated Servers

Dedicated servers is usually the best way to prevent cheating, a tertiary server is added to the mix, and said server is either controlled by the game creators or uses a software specifically tailored to work as a "multiplayer server".

![Example of a dedicated server](./images/balancing/dedicated.pdf){width=60%}

Such server contains the authoritative game state and decides what is right and what is wrong, or either what is possible and not possible.

![Two cheaters vs. Dedicated server](./images/balancing/cheat_dedicated.pdf){width=60%}

Usually a dedicated server software has been specifically made to limit cheating, as well as offering better performance than the P2P counterpart (it doesn't have to run graphics, for instance).

If a consistent leaderboard and lack of cheating is important (for instance in a heavily competitive multiplayer game), you should probably choose this option.
