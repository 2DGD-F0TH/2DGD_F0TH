A primer on Cheating
--------------------

Cheating is the act of fraudulently altering the game's mechanics in advantage of the player, performed by the players themselves.

It is something that many game developers and designers have to battle against, so here are some suggestions and tips to limit cheating in your game.

This section will just give a primer on the types of cheating we can find, since knowledge of something is the best weapon against it; so questions like "how to cheat" (or "how to hack") are outside the scope of this book.

### Information-based cheating

Information-based cheats are all those cheats that rely additional information to the cheater, such information can give a sizeable advantage. A possible example is a cheat that removes the so-called "Fog of War" in a real-time strategy (RTS) game: having possibility of seeing all the enemy units allows the cheater to put up some countermeasures against the units that are being created.

These cheats include also x-ray hacks, all cheats that invalidate invisibility (as the server or peer would still need to transmit the coordinates of the hidden unit) and anything that can show information that is not meant to be shown to the user.

A possibile solution is for the game to just "not transmit" the data, making the cheat useless, but sometimes that is just not possible.

### Mechanics-based cheating

Another category of cheats is comprised of all those hacks that alter the game mechanics themselves, like killing all the players on the map. These kind of cheats are usually made possible by exploits or just because the cheater owns the server people are playing on.

These kinds of cheats can easily hinder the playability of a game, or even make it outright unplayable.

A possible solution to these cheats would be using a cheat-detection program (which would start a "cat and mouse" game, where hacks are updated to avoid detection, and detection programs are updated to detect new hacks) and also inserting some client-side verification of server commands (in case the server contains the "authoritative game state"); for instance if all players are killed at the same time, the clients could flag the server as possibly cheating.

### Man-in-the-middle

This attack is well known in the networking environment: an external machine is used to route and intercept all the traffic directed to the game. This can be a real issue since the attacking program is "outside of the game environment", making nearly all cheat-detection programs useless.

A man-in-the-middle attack can also be used to further exploit the game and find new vulnerabilities.

A possible solution could be completely encrypting all the game's traffic, but that will be an issue since encryption takes away precious CPU cycles, and this could lead to an hindered gaming experience.

### Low-level exploits

These kind of attacks don't target the game itself, but tend to attack the technology that the game is using: they can range from brute force attacks (like using Denial of Service attacks) to more articulated actions that may break the equilibrium of the game.

This is not your game's fault: it's a problem with the technology stack used, the libraries or even the fact that the software is running on a computer. These exploits are really engrained into computers themselves, but they can still be mitigated with a little bit of care and attention.

How cheating influences gameplay and enjoyability
-------------------------------------------------

### Single Player

Cheating in single player is an act that doesn't usually do a massive amount of damage, and such damage is usually confined inside the single "single-player" game.

Playing outside of the rules can be really fun (that's one of the principles the "glitch hunters" love: doing something outside of what another person imposed them), for instance some people cheat in games to bring some mayhem into their gameplay, or they use cheats implemented inside the game itself for a comedic factor (like the omnipresent "giant head" cheat).

Sometimes cheating happens because the game is unbalanced and players get annoyed at it, an instance of this happening could be when a game has a great story and gameplay but there is a boss that is so hard the game just stops there. You want to see how the story continues, but the game has gone so much out of balance you are willing to break its own mechanics to be able to continue it.

In this case the approach you should have is rebalancing the game, instead of limiting your players.

When it comes to cheat prevention, usually the first order of action is giving the game the ability to "check the validity" of an instruction.

For instance if a player character has its coordinates at (5,5) on frame $n$ and coordinates at (1500, 5) at frame $n+1$, there is something fishy going on, since maybe the player can only move 500 pixels per second (while it moved 995 in one frame: $\frac{1}{60}$ of a second).

Such checks will slow down the processing, but will allow you to put a limit to cheating, possibly intervening in an active way, by resetting the space walked to the maximum amount possible in one frame, although this could give some issues with slower computers and [variable time steps](#variable_timesteps).

### Multiplayer

When it comes to multiplayer and "leaderboards", cheating can be create some major damage to the game's enjoyability. It is honestly disheartening seeing a level that has been completed in 0 seconds on top of the leaderboard, totally unreachable with normal gameplay.

When competitive gameplay comes into the picture, playing against a cheater is frustrating and maddening, you feel powerless, the game is not fun and sometimes it even feels "broken", even though it is stable and playable.

Here we will distinguish between the two main forms of multiplayer: Peer-to-peer gameplay and dedicated servers.

#### P2P

Peer-to-peer multiplayer is the economically cheapest and easiest way to implement multiplayer, two or more computers (or consoles) are on "the same level" and communicate directly with each other, without a tertiary server in the middle.

![Example of a P2P connection](./images/balancing/P2P.svg){width=60%}

The main difficulty in preventing cheating is that there is no "authoritative game state", the program cannot know if either player is cheating besides having an array of "possible actions", like in single player, but with the added difficulty of network lag.

![Two cheaters meet in P2P](./images/balancing/cheat_P2P.svg){width=60%}

Giving such "authoritative game state" to either of the players is not a good idea, because that way they would be able to cheat themselves and since they're the "game master", everything they do would be accepted.

![What would happen if one of the Peers had the autoritative game state](./images/balancing/auth_P2P.svg){width=60%}

This is also the reason why many games that make use of P2P connections have implementations of anti-cheat systems that are shoddy at best.

#### Dedicated Servers

Dedicated servers is usually the best way to prevent cheating, a tertiary server is added to the mix, and said server is either controlled by the game creators or uses a software specifically tailored to work as a "multiplayer server".

![Example of a dedicated server](./images/balancing/dedicated.svg){width=60%}

Such server contains the authoritative game state and decides what is right and what is wrong, or either what is possible and not possible.

![Two cheaters vs. Dedicated server](./images/balancing/cheat_dedicated.svg){width=60%}

Usually a dedicated server software has been specifically made to limit cheating, as well as offering better performance than the P2P counterpart (it doesn't have to run graphics, for instance).

If a consistent leaderboard and lack of cheating is important (for instance in a heavily competitive multiplayer game), you should probably choose this option.

This section assumes that the game is using a third-party game server, and none of the players has direct access to said server, as this would enable cheating on the owner's part.

Cheating protection
-------------------

Protecting your game, leaderboards and community from cheating is hard and there are many ways to prevent it. Dedicated servers are one of the many ways, cheat engine detection (there are many commercial solutions) but none of these solutions is cheater-proof.

If your game has 1 million players (I hope it does!) and there is a 0.0001% probability of people cheating at it, you have a statistical certainty that there will be a cheater among them. Cheating will happen and usually cannot be completely prevented, but that shouldn't discourage you.

It should be hard to cheat at a game but, most of all, it should be harder to do it undetected.

So there are some other tools in your toolbox that you can use, for instance you can save a "lightweight" replay of the game session (if your game has a leaderboard), that way anyone who wants to enter the leaderboard will also send a replay of their gaming session.

This has multiple advantages: from the community side you have players can "learn tricks from the best", but also can report who evidently cheats, because the replay would show it.

Usually these "lightweight" replays are done by recording the position of the player and its state, as well as the initial state of the game. Add the fact that the game is deterministic and you have the equivalent of a recording of the gameplay.

Even better, you can record the inputs of the player and see if "a simulation" done with those inputs validates against the positions and actions recorded on the replay: this way if someone modified their game to make themselves invincible or faster, the simulation would fail and the replay wouldn't validate.

::: trivia :::
It was recently discovered that the game "Trackmania" records inputs as well as the position of the vehicle, this allows the game to validate the replays against the most common forms of cheating
::::::::::::::

We can be sure that the simulation would be equal to a video because a game (as well as any program) is deterministic: given the same initial state and inputs, the game will always end up the same way. This is true even if random numbers are involved (that is why they're called "pseudorandom"), see ["random numbers on computers are not really random"](#random).

### Debug Mode vs Release Mode

Sometimes it may prove useful (and sometimes it is just plain necessary) to have some "cheats" to use as "shortcuts" while doing some development tasks.

Some of these give you invulnerability to make sure that you can test the balancement of the weapons without worrying about dying in the harder stages, some other times you need a "level skip code" to quickly get to later levels when you have made incompatible edits to the savefile format.

When using these codes it is vital to have a build flag to distinguish between a "release build" and a "debug build", this way it is possible to completely strip out the debug code from the build, thus "reducing the attack surface": you can't abuse code that is not there.

This is more difficult when dedicated servers are involved, since it would be necessary to have 2 copies of the server: a "release build" with all the debug code stripped out, and a "debug build" that allows for "debug cheats". Problem is that not having such "cheats" could make moderators' work a lot harder (let's consider cases like Minecraft servers, where administrators need to be able to fly around to be able to scout possible cheaters "by eye").

Some common exploits
--------------------

In this section we will take a look at some really common exploits that can be used to break the balance of a game. By knowing these kinds of exploits you will be able to plan ahead and avoid annoying (or embarrassing) situations.

### Integer Under/Overflow

This is probably one of the most common exploits that you can find in games. Computers have limited memory, thus they have a limit on the numbers that can be represented: what happens when such limit is exceeded?

#### Two's complement

First of all, integer numbers are saved on our computers in "two's complement", which requires the most significant bit to be reserved for the sign of a number. I won't explain deeply how the two's complement conversion works, but remember the following: to perform a two's complement you

1. Flip all bits
2. Add 1 to the number you obtained in step 1

Let's see a simple example with 3 binary digits:

| Decimal | Binary |
| :------ | :----- |
| -4      | 100    |
| -3      | 101    |
| -2      | 110    |
| -1      | 111    |
| 0       | 000    |
| 1       | 001    |
| 2       | 010    |
| 3       | 011    |

#### How the attack works

Let's imagine a simple management game: you can earn money from various activities and you spend money on staff wages. There is a button that allows you to increase staff wages by 1 unit. Staff wages are saved using (for some reason) only 3 binary digits.

The game calculates your monthly earnings as follows:

$$
earnings\ =\ income\ -\ wages
$$

Now that we have set up the environment, let's see how it can be broken.

Let's assume we have only one staff member how is paid 3 units per month. What would happen if we paid them another unit? In decimal it would be easy: $3+1 = 4$ but in binary it is a lot more complicated:

$$
011 + 001 = 100
$$

If we look at the previous table, that's the two's complement representation of $-4$! By raising the staff wages we've ended up with the staff paying us! This is called an "integer overflow".

This can work both ways: if we subtracted 1 from $-4$ in binary, we would go back to $+3$. This is what happens when an integer "underflows".

:::: trivia ::::
There is a story about the first "Civilization" game having an integer underflow bug (called "Nuclear Gandhi"): a civilization's "aggression value" was saved as an 8-bit unsigned integer. Gandhi had its initial aggression score set as 1 and when India achieved democracy, such score is lowered by 2 points, causing the "aggression value" to underflow to 255, making Gandhi the most aggressive leader in the game.

This is actually not true, but "Nuclear Gandhi" was included as an Easter egg in the following games, as a joke.
::::::::::::::::

### Repeat attacks

This affects for the most part multiplayer games that make use of the internet. Let's imagine the following situation: you have an online game that follows this procedure:

1. You accept a mission;
2. The mission is added to your journal;
3. You perform the mission's tasks;
4. You turn in the mission;
5. You receive experience and gold from the mission;
6. The mission is removed from your journal.

Between points 4 and 5 there is a synchronization effort between your client and the dedicated server: if the server doesn't confirm that the mission is really turned in, no experience is received and the mission stays in the journal, ready to turn in.

What if there was no control (on the server side) for turning in the mission more than once? This is what repeat attacks exploit.

In our example, you would get a program that throttles the connection to the point that the network code of the game is suffering heavily, but doesn't disconnect. At this point you just turn in the mission a lot of times (and such mission will stay in the journal because the client didn't receive an answer from the server yet). Since the server doesn't have a check for multiple mission turn-ins, it will return orders to the client for adding more experience and gold.
