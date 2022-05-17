{{pagebreak}}

Balancing Your Game
===================

:::::: {.epigraph author="Simon Sinek"}
The trick to balance is to not make sacrificing important things become the norm
::::::

An imbalanced game is a frustrating game, and most of the time balancing a game is one of the toughest challenges a game developer/designer can find themselves to have to face.

Let's talk about some principles and guidelines that can help you balancing your game and keep your players challenged but not frustrated.

Do not annoy the player
-----------------------

The "master principle" everyone should follow (in my humble opinion) is "do not, under any circumstance, annoy the player".

You should not trade the "fun" of your game for any other mechanic (like showing an advertisement to allow them to continue playing): that is equivalent to betraying your player, makes the game feel unfair and un-fun.

Here are some examples of mechanics that will surely annoy the player:

- **Sudden spikes in difficulty:** when you have a sudden spike in difficulty, the player feels stumped and the game tends to lose its charm, you are "interrupting the flow" of the game by placing an arbitrary hurdle on your players' road;
- **Off-screen instant-death traps:** having something deadly that pops out from off-screen and kills the player is unfair and will make your players scream in agony and vexation, if you want to place some obstacles that pop from off-screen you should "telegraph" them. "Telegraphing" is a technique where you send a warning signal to the player that danger is coming. For instance a huge laser that instantly kills you should be preceded by a couple seconds by a yellow "!" signal on the right side of the screen, where the laser is due to strike. Another way to telegraph said laser would be to illuminate the part of the screen that is about to be hit, like the light of the laser is coming up;
- **Arbitrary invisible time limits:** If you suddenly interrupt the player's game with a "time up" and you have no countdown on the screen, the player will get frustrated, guaranteed;
- **Taking control away from the player:** Not allowing the player to move (getting blocked by an enemy and killed) or just not allowing the player to adjust their jump mid-air is a surefire way to make them not play your game anymore.

Difficulty curves
-----------------

When designing our game, it may be useful (sometimes mandatory) to have a high-level view of how our game's difficulty will evolve as the game itself is played. If we take a cartesian plane and define time as the $x$ axis, while the "perceived difficulty" is plotted on the $y$ axis, we would obtain a **difficulty curve**, a high-level representation of how difficulty evolves as the game is played.

Knowing some basic difficulty curves, as well as their pros and cons, may give you an idea of how you want to build and balance your game. This section will be heavy on charts, so be prepared!

### Simple Lines

Let's start with simple lines, they can be straight lines or simple curves that don't feature any waviness or wobbliness. These are usually the simplest to learn but that doesn't mean they are free from complicated drawbacks. Let's check some out.

### Flat Line

The first curve is the "flat line", which is a simple horizontal line that spans the whole playtime. It can't get simpler than that.

![A Flat line difficulty curve](./images/balancing/flat_line.svg){width=40%}

When the player selects a difficulty level, the difficulty stays around that value (with no real perceivable change) for the entire playthrough. That means that there is no "evolution" to take care of and no long-term balancing to perform.

This curve also represents a way of balancing your game that gets boring rather quickly, since the player gets better at the game as time passes, but the game doesn't "follow them" by giving them a higher challenge.

#### Linear Increase

To solve the issue of the flat line, you can add a linear increase to your difficulty in an effort to "keep up" with the player.

![A linearly increasing difficulty curve](./images/balancing/increasing_line.svg){width=40%}

This curve is usually easy to manage, giving a lot of control over the initial difficulty and its evolution. The player is challenged for longer periods of time, since the game becomes more difficult the further the player plays it.

The biggest drawback of this kind of curve is its predictability: after a while, a somewhat "expert" player can predict "by feel" the upcoming challenges and prepare as a consequence, thus "squashing" the final part of the curve.

![As the player learns to predict, the difficulty curve changes from our design](./images/balancing/increasing_line_squashed.svg){width=40%}

#### Logarithmic Line

The logarithmic line is usually presented as a "guide" for more advanced types of curves. This is due to the fact that it has some glaring problems.

![A Logarithmic difficulty curve](./images/balancing/logarithmic_line.svg){width=40%}

The beginning of the game has a steep learning curve, which eases up as the game goes on. This means that the game is really hard at the beginning but the challenge dies down towards the end, which can make for a game very difficult to learn but not much more.

This can be a good curve if you want to "test the might and patience" of your players, but if not paired with a different approach in the late game, it may end up being plain boring.

#### Exponential Line

The complete opposite of the logarithmic line is the exponential one. This has a lot more use in game design that the previous example.

![An exponential difficulty curve](./images/balancing/exponential_line.svg){width=40%}

The exponential difficulty curve gives the player a very relaxed beginning as well as a late game that can get really hard really fast (to the point that it can be too hard). This curve is the literal definition of a game that is "easy to learn but hard to master".

### Wave patterns

The difficulty curves that we've seen so far have all one thing in common: they are simple and feature no real "lack of predictability", which can make a game a bit boring in the long run. Not because it's not challenging, but because it's predictable.

#### Linearly Increasing wave

Adding some waviness to the linearly increasing line can add some spice to the game very easily.

![A linearly increasing wavey difficulty curve](./images/balancing/increasing_wave.svg){width=40%}

This is a very efficient way of working, since it makes things more interesting, but if not implemented correctly it can lead to very high difficulty during the late game, since the "wave" may compound with an already high difficulty level.

#### Logarithmically Increasing wave

To try and fix the issues from the linearly increasing wave pattern, we may want to tie our difficulty to a logarithmic line.

![A Logarithmically increasing wavy difficulty curve](./images/balancing/logarithmic_wave.svg){width=40%}

This kind of difficulty curve tends to "squash" the challenge towards the mid-to-late game, thus making the game a bit less difficult if the "wave" compounds with an already high difficulty level.

As a drawback, this curve may feel more "predictable" towards the late game, since the difficulty tends to get very "horizontal" towards the end.

### Interval Patterns

For games that involve some random generation, like roguelites, we may want to "clamp" the difficulty between a "minimum" and a "maximum" but still allow for "runs" that feel different in difficulty from each other.

In this section we will show only wavy patterns, to exemplify the most "difficult to design" patterns, butt all patterns apply to simple lines too.

#### Simple Interval

{{placeholder}}

![A simple wavy difficulty interval](./images/balancing/simple_interval.svg){width=40%}

{{placeholder}}

<!-- TODO: Impredictable pattern, good for having impredictable challenges. Gives no real control over the initial difficulty (so you may have a run that starts really hard) -->

#### Widening Interval

{{placeholder}}

![A widening and wavy difficulty interval](./images/balancing/widening_interval.svg){width=40%}

{{placeholder}}

<!-- TODO: Allows for better control over the initial difficulty, as the wave widens as the game continues. May become too hard or too easy towards the end, due to the fact that the wave widens as the game continues -->

#### Widening Interval with Logarithmic trend

{{placeholder}}

![A widening wavy difficulty interval with a logarithmic trend](./images/balancing/widening_interval_log.svg){width=40%}

{{placeholder}}

<!-- TODO: Tries to solve the issue of "too easy or too hard towards the end" by tying the difficulty to a logarithmic line. This way the game has a somewhat "increasing trend" while still giving an unpredictable challenge. Can become difficult to control. -->

### This is not everything

{{placeholder}}

<!-- TODO: Exemplify different curves (sawtooth especially), tell the reader to mix and match, represent graphically what to do and what to avoid. Also explain that difficulty is not everything, that comedy and other factors can be a source of fun -->

#### Sawtooth patterns

{{placeholder}}

![A sawtooth difficulty curve](./images/balancing/sawtooth_line.svg){width=40%}

{{placeholder}}

<!-- TODO: Good to give some variety to the gameplay: every time a new mechanic/ability is introduced, the game gets a little bit easier. Then it gets more difficult, until you reach a difficulty higher than the last time you got an ability. Then new ability, a bit easier, etc... -->

#### What not to do

{{placeholder}}

<!-- TODO: Avoid difficulty spikes. Avoid making the game easier for who is going well (add secret skill points in an area only expert players can reach), don't punish players further than strictly necessary if they do bad (losing a life is already enough, losing all gear without possibility of recovery is even worse), don't let players skip learning skills (this will make the game a lot harder down the line), don't overload the player with info -->

Always favour the player
------------------------

In the process of balancing a game, as a game developer/designer you will surely find yourself in front of the following decision time and time again:

> Shall I favour the game's precision or should I give some leeway to the player?

The answer is always the latter.

Giving some leeway to the player, for instance by having a more generous hit-box that allows you to stay alive even if a bullet grazes your character makes the game seem more "fair".

There are infinite ways to make a game challenging without having to force the player into accepting very precise hit-boxes or extremely tight gameplay.

Economy
-------

Some games (not only MMOs) feature an "economy" side to their gameplay: this can prove to be something really difficult to balance without creating a virtual financial disaster.

This section will give you some basics to get things right.

### Supply and Demand

Every economy is (at least in part) governed by the laws of supply and demand, which can be graphically represented in the following graph:

![A simplified vision of supply and demand](./images/balancing/supply_demand.svg){width=50%}

:::: note ::::
This is an oversemplification of how the market (and the economy in general) works, just enough to keep you far away from the most common issues.
::::::::::::::

We can get some takeaways from such graph:

- If the demand is low (noone wants the product), suppliers will try to "boost it" by lowering prices;
- If the demand is high (many want the product), suppliers will try to earn more by boosting prices;
- If the supply is low (the product is rare), people will value it more (the price will be higher);
- If the supply is high (the product is really common), people will value it less (paying it less).

This also shows that artificially keeping the supply low will make the product feel more valuable, allowing to ask for higher prices.

Another thing to remember: money is a good too, and is subject to the same laws.

### Money sources and sinks

Any artificial economy is usually composed by 2 "components":

- **Money Sources:** they create money from nothing, these can be quest givers, treasure chests and the like;
- **Money Sinks:** places that "destroy money", these are NPC salesmen at the market (that create items from nothing), fortune machines, anything that takes or exchanges money for something else.

Sources and sinks are extremely important and should be carefully balanced, since an imbalance in the quantity of money created and destroyed can have catastrophic effects. Among those, uncontrolled inflation and deflation are the most prominent.

### Inflation

Inflation is a phenomenon where prices usually rise uncontrollably: this means that money "lost its value".

This is usually due to the massive presence of money in the economy, so in a source/sink view, the money sources emit much more money than what the sinks can consume.

As a consequence fixed-price operations (like if you put "repair a weapon" at a fixed 50 golds) become incredibly cheap, while products in the market become prohibitly expensive.

In a supply/demand perspective, there is big supply of money which triggers little demand for it (since it's so common), while there is a big demand for products (thus raising the prices).

This may end with people having loads of money and noone accepting them for trades. Bartering may arise as an alternative to money.

### Deflation

Deflation is a phenomenon where prices usually have a drop: this means that money has "too much value".

This has the exact opposite causes of inflation: there is too little money in the economy, so the money sources don't emit enough money and there are too many sinks that can consume it.

As a consequence fixed-price operations become extremely expensive (if you have 100 gold, paying 50 gold to repair a weapon may seem a lot), while products in the market become extremely cheap.

Again, in a supply/demand perspective, there is a low supply of money (making it more valuable), while demand is really high.

This can trigger "money hoarding" thus freezing the economy, sometimes bartering can arise as an alternative way to exchange goods without involving the "precious precious money". Some operations that require a minimum amount of money may even get locked because of the little amount of money circulating.

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

This is more difficult when dedicated servers are involved, since it would be necessary to have 2 copies of the server: a "release build" with all the debug code stripped out, and a "debug build" that allows for "debug cheats". Problem is that not having such "cheats" could make moderators work a lot harder (let's consider cases like Minecraft servers, where administrators need to be able to fly around to be able to scout possible cheaters "by eye").
