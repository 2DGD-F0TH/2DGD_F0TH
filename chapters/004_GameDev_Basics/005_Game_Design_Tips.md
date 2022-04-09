{{pagebreak}}

Game Design Tips
================

:::::: {.epigraph author="Milton Glaser"}
There are three responses to a piece of design - yes, no, and WOW! Wow is the one to aim for.
::::::

Game design is a huge topic, in this section we will just dip our toes into the argument, talking about some genres and features in games, including some tips and tricks that can make the difference between a "good" and a "bad" experience.

In this section we will also talk about level design tips, tricks and common pitfalls to avoid. We will talk about tutorials, entertaining the player and ways to reward them better.

Tutorials
-----------

### Do not pad tutorials

Tutorials are meant to introduce the player to the game's mechanics, but a bad tutorial can ruin the experience. Tutorials should be comprehensive but also compact, padding tutorials should absolutely not be a thing.

Gloss over the simpler things (usually the ones that are common to the genre) and focus more on the unique mechanics of your game.

Avoid things like:

> Use the "right arrow" button to move right, the "left arrow" button to move left, use "up arrow" to jump, use "down arrow" to crouch

Instead use:

> Use the arrows to move.

And eventually present the more complex mechanics in an "obstacle course" fashion.

### Integrate tutorials in the lore

Tutorials are better when well-integrated in the lore, for instance if your game features a high-tech suit maybe you should make a "training course" inside the structure where such suit was invented.

By integrating the tutorial into the game world, it will feel less of a tutorial for the player, but more like training for the game's protagonist.

### Let the player explore the controls

Sometimes it's better to allow the player to explore the controls, by giving them a safe area to try: this area is usually a tutorial or a specific training area.

It can prove more effective to avoid spoon-feeding your player with all the moves, and just let them explore the core mechanics of the game by themselves, eventually assisted by an in-game manual of some sort.

So instead of doing something like (thinking about a 2D tournament fighter):

> Do $\rightarrow \searrow \downarrow$ + A to do a chop attack
>
> Do $\rightarrow \nearrow \uparrow$ + A to do an uppercut
>
> ...

Try something like:

> Do $\rightarrow \nearrow \uparrow$ + A to do an uppercut
>
> Try more combination with your arrows and the attack buttons for more moves
>
> Check the move list in the pause menu

Consolidating and refreshing the game mechanics
------------------------------------------------

### Remind the player about the mechanics they learned

There's a latin saying that goes "repetita juvant", which means "repeating does good".

A good idea is to sprinkle around different levels concepts that have been learned previously, so to remind and consolidate them. This is more effective when done shortly after learning a new mechanic.

### Introduce new ways to use old mechanics

After a while, old mechanics tend to become stale, to rejuvinate them we can apply such mechanics to new problems. Changing their use slightly can make an old experience new again.

For instance, knowing that shooting our magic beam against something on the ceiling will make it drop (usually killing an enemy), we can make the player use such envinronmental interactivity to drop a suspended weight to open a door, or shoot a bell to "force" a change of guard so to sneak stealthily.

Rewarding the player
---------------------

### Reward the player for their "lateral thinking"

A good idea could be rewarding the player for not throwing themselves "head first" into the fight, but instead thinking out of the box and avoid the fight altogether, or just win it differently.

Putting a very powerful enemy in front of some treasure (for instance currency used in-game) can seem unfair, unless you place an unstable stalactite that can be shot with your magic beam.

Your magic beam won't deal enough damage to the enemy to kill it before such enemy takes your life, but a stalactite on their head will do the trick, and the reward for such lateral thinking will be a heap of coins (or gems, or whatever currency you invented).

![Example of how to induce lateral thinking with environmental damage [^chests] [^icetiles] [^undeadfossil]](./images/game_design/lateral_1.png){width=40%}

Giving tips to the player by breaking the fourth wall can be another idea, a rock or a patch of dead grass conveniently shaped like an arrow could point towards a secret room that has a fake wall.

![Example of how to induce lateral thinking by "breaking the fourth wall" [^jawbreaker]](./images/game_design/lateral_2.png){width=40%}

This last tip should be done very subtly, so not to ruin the immersion. Unless your game takes advantage from these kind of things (for instance games based on comedy).

### Reward the player for their tenacity

After suggesting to reward players for not butting head-first into fights, now I'm going to suggest the exact opposite (in a way): reward your players for their tenacity.

Beating a tough boss with a certain (weak) weapon, or just the plain tenacity and skill that is needed to undertake a hard task, such feats should be rewarded: for instance with a powerful weapon that can be used after some level-ups.

### Reward the player for exploring

Exploration can lead the player to discover secrets, which can range from simple gear, to pieces of unexplored environment, or even pieces of the game's lore.

World exploration should not be limited to simple secrets, a nice idea could be finding a path towards something that is usually considered "environmental damage" (like a catapult in the background) so that the player can deactivate it.

Thinking out of the box can lead to some really interesting results when it comes to this tip.

### Reward the player for not immediately following the given direction

This is an extension of the previous point, the player should be rewarded for their exploratory efforts, even more when those efforts mean not immediately following the direction given by the designer.

"Thinking differently" should be rewarded with challenge and rewards up to said challenge. If the mission tells a player to climb up a tower, the more curious players could be led to hit the tower's underground dungeon before going on with the mission. A nice challenge in such dungeon with a fittin reward could expand on the game experience.

### Reward the player for not trusting you entirely

Sometimes it can be fun, for both the game designer and the player, to play a bit of a trick to the player themselves.

Some famous games, like DOOM and Dark Souls, use secrets-in-secrets to trick players into thinking they found something valuable, while hiding something way more important. Let's see the example below.

![Example of secret-in-secret](./images/game_design/secret_in_secret.png){width=50%}

We can see how we hid a secret inside of another secret and used a piece of valuable (but not *too valuable*) treasure to make the player think they found the secret, while the real secret is hiding behind another fake wall.


### Reward Backtracking (but don't make it mandatory!)

To make the game's experience broader and richer, you may want to reward the player's exploration efforts by hiding treasure behind some backtracking.

For instance you can show the player a locked door somewhere in the level, such door will unlock and open after beating a boss monster or a wave-based challenge in the next room and hide some weapons that would otherwise be unlocked further into the game.

A nice idea would be "suggesting" to the player that something interesting happened, by playing the sound of the door opening as soon as the event is triggered. Another idea would be showing the player that the door opened (for instance if you're in an open area, the player would be able to see clearly that an open gate that was closed before).

The most important thing to remember is that all of this needs to be optional, a reward for the player's willingness to explore your levels further: avoid making backtracking mandatory, this will only feel like you're "padding the game" with nothing worth of note.

The player is paying you with their time and effort, it's only right that you pay them back with a pleasurable experience.

::: trivia :::
A nice example of backtracking bonuses (although mandatory to 100% the game) is used in the level "Sphynxinator" in Crash Bandicoot 3: Warped.

When you start the level, you can run backwards and you'll find 4 crates (which are necessary to get the "Gem" and 100% the game, but not mandatory for the normal ending), one of which is an extra life.

The backtracking is really short and gives you a nice bonus.
::::::::::::::

### The "lives" system

Extra ships, 1-ups, extends, continues: these are all instances of what we can call the "lives system". This system gives a more "arcade feel" to your game and adds an important challenge factor to it.

Without something that threatens a game over, beating the game is no longer a challenge, but it's a matter of time. When overcoming a challenge is inevitable, it is not a challenge anymore, and the player will end up losing interest.

This is what the "lives system" is for: it's a "sword of Damocles", hanging over the player's head, continuously threatening a "game over" and pushing the player to do their best in order to get as far into the game as possible.

"Continues" are just "a lives system for your lives", they're in a very limited number (or have a price, like putting another quarter into the arcade cabinet) and allow you to "continue the game" with a new set of "lives" without losing your progress.

As with all things in videogames, it doesn't need to bring real challenge, but just the "illusion" of it.

Furthermore, lives and continues are a great tool to reward your player for their efforts: giving them an extra life every 20.000 points, granting a continue for a no-hit boss battle, putting a bunch of 1-ups in a hard-to-reach place are all great ways to challenge and reward your player and give your game more depth.

#### 1-UPs

When a life system is in place, getting an extra life (a so-called 1-UP) is cause for celebration, since it allows the player to get further into the game or play with new and bolder strategies or just feel more at ease.

There are many ways you can reward the player with an extra life, such as:

- Finding a secret;
- Reaching a certain score threshold (for example every 100.000 points);
- Finding a certain item (a "physical 1-UP");
- Complete a certain combo-chain (for example kill over 8 enemies without touching the ground);
- ...

No matter how the 1-UP is achieved, this should be celebrated with a jingle that is very recognizable: this will allow the player to "know" that they got a 1-UP without thinking too hard about it. Not "celebrating" this event would make it "ordinary" and uninteresting, while it's extremely important in the grand scheme of things.

Some games even go as far as temporarily pause the game while the (short) jingle plays, that how important an extra-life is: "Stop everything! We got a 1-UP here!".

{{placeholder}}

<!-- TODO: Talk about other level design tips -->

Loading Screens
---------------

Loading screens can be subject to design too and deciding what to put in them can really enhance the player experience with your product.

### What to put in a loading screen

"What can we put in a loading screen?" The answer may sound obvious to some, but a simple loading screen has a lot going on. Let's think about it, the most barren loading screen imaginable has at least two elements:

- An animation, to make the loading screen less boring and to ensure the player that our game didn't lock up;
- Some kind of progress indicator, to let the player know how far the loading routine has gone.

But we can put lots more into it, let's take a

- **Story:** In story-heavy games, it may be a good idea to put a reminder, a briefing or just a couple sentences telling what's happening story-wise. This was done in DOOM (2016).
- **World Building:** If the story is not-so-linear, a good idea could be just telling some facts about the world of the game, what some primary NPCs like, their habits, etc...
- **Tips:** Putting some tips to help the player is one of the most common things done with loading screens, these tips should be short and useful (no "press F to kill your enemy silently", that's basic controls).
- **Minigames:** If your game may take a potentially long time to load, making the player play a simple mini-game (maybe with rewards) while they wait. This was done on the Playstation 2 (and can be enabled on the PC version) of Okami, where you can earn demon fangs with two minigames.
- **Status-related sentences: ** This is something that can serve both as a "loading screen filler" as well as "hidden debug feature"
    - **Actual loading information:** Some players may like knowing what their PC is doing, so showing "loading backgrounds" or "loading sound data" is a nice screen filler and can give your players a pointer in case the game locks up while loading.
    - **Funny phrases: ** Instead of boring, actual loading information, you can put funny phrases like "inserting buckazoids" (Space Quest, anyone?), if you have a list of funny phrases connected to actual loading checkpoints, it will function as a small debug helper.

### Letting the player "exit" the loading screen

In some cases it can be a good thing to have a loading screen fade into the next stage (or area) directly, while in other situations it may be wiser to prompt the player to "exit" the loading screen themselves (maybe by a "Press any key to continue" prompt).

The most important factor is whether the areas we will load into are safe: if the player is not ready, they may get killed by enemies, and that feels unfair.

You should use a "press to continue prompt" when at least one of the following conditions applies:

- **Any area we load to may be unsafe:** We don't want our players to take a small break, walk around the room and come back to a dead character;
- **There is text on the loading screen:** Be it a tip, world building or story, the player may be reading it, and taking away the text will annoy them.

### Avoiding a loading screen altogether

{{placeholder}}

<!-- TODO: Dynamic loading: some games use closed rooms with nothing in them (double door) to unload the old map and load the new one dynamically without forcing a loading screen -->

Designing the story and gameplay flow
-------------------------------------

When we are preparing the terrain for our game, it is vital to have an idea of how the story and the gameplay will unfold. There are lots of different types of gameplay, here we present some of them.

### Linear Gameplay

This is the simplest type of gameplay design: all story events come one after the other, without any possibility of deviating from the flow.

![Example Scheme of linear gameplay](./images/game_design/linear_gameplay.svg){width=60%}

Very much like a presentation, there is no branching, but such linearity can present some advantages, like ease of testing and possibility of applying traditional storytelling tools which have been developed for thousands of years.

Table: Summary of linear gameplay

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Linear Gameplay

**Advantages**            Simple and cheap to test, traditional storytelling tools can be used easily.

**Disadvantages**         No replayability, the gameplay may not feel very "interactive"
------------------------------------------------------------------------------


### Branching gameplay

Going towards more complex flow types, we can use branching to allow for more interactivity.

![Example Scheme of branching gameplay](./images/game_design/branching_gameplay.svg){width=50%}

This type of gameplay flow allows for a lot of interactivity by crafting the game in a way that player decisions have a direct influence on the story flow.

This gameplay flow is harder (and thus more costly) to test, but allows for multiple endings.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Branching Gameplay

**Advantages**            Simple to implement, allows for a strong feel of interactivity, allows for a lot of replayability by giving gameplay paths.

**Disadvantages**         Hard and costly to test, can get out of hand if not managed correctly.
------------------------------------------------------------------------------

### Parallel gameplay

The Branching gameplay flow has a huge disadvantage: it can be really hard to manage and doesn't really suit well "more linear" games.

Here's where parallel gameplay comes into play.

![Example Scheme of parallel gameplay](./images/game_design/parallel_gameplay.svg){width=60%}

In this flow style, there are branches running "parallel" to one another, but merge into "mandatory events" (which are usually story related). This way we have varied gameplay while keeping the story essentially linear.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Parallel Gameplay

**Advantages**            Moderately expensive to test, some traditional storytelling tools can be used, story is easier to manage.

**Disadvantages**         Replayability suffers from a story standpoint. If not well-made the player will feel like the story is "on rails" from the get go.
------------------------------------------------------------------------------

### Threaded gameplay

A different kind of gameplay is the "threaded" version, where there are many "beginnings", "middles" and "endings", usually done by playing different characters.

![Example Scheme of threaded gameplay](./images/game_design/threaded_gameplay.svg){width=60%}

This gives more replayability by giving many different and intertwining stories that allow to better understand a "bigger picture" of some sort. This gameplay flow can be costly, since it requires testing all the possible paths and crossings.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Threaded Gameplay

**Advantages**            Good replayability, great for giving many "sides" to a story.

**Disadvantages**         Testing all the paths can be costly, more difficult to manage.
------------------------------------------------------------------------------

:::: trivia ::::
This was done in Resident Evil 6, where different characters (and teams) have different stories that overlap.
::::::::::::::::


### Episodic gameplay

A more "object-oriented" approach to storytelling can be done by making small "episodes" (like mini-stories) with many entry and exit points.

![Example Scheme of episodic gameplay](./images/game_design/episodic_gameplay.svg){width=60%}

We need to be mindful of loops (we don't want to replay an episode that was already completed) when laying out our episodes. This gameplay flow allows for great interactivity, but kind of "forces" replaying the game to see all the possible episodes.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Episodic Gameplay

**Advantages**            Great interactivity.

**Disadvantages**         Tends to "force" replaying the game to see all episodes and paths, hard (and thus costly) to test and manage.
------------------------------------------------------------------------------

### Adding parallel paths

Nothing forbids us to mix and match methods to create something that suits our game better.

A very much appreciated and used gameplay flow is having a linear story with lots of "side quests" to give some diversion from normal gameplay, as well as replay value, since people are bound to miss some side quests.

### Looping Gameplay

{{placeholder}}

<!-- TODO: Typical of roguelike games, each game is a "run" which helps you progress in a "bigger story" (like in the style of Binding Of Isaac) where each run changes the following ones by adding objects, bosses and levels -->

Some game genres and their characteristics
------------------------------------------

### Roguelikes and Rogue-lites

Roguelike games are usually games that involve dungeon-crawling and procedurally generated levels, usually with a fantasy background. In this small section we will take a look at the features that characterize roguelike games.

The most accepted interpretation of a roguelike game is the "Berlin Interpretation", which is based on the features that follow. When games diverge from these features, but are still loosely based on the classic roguelike design, they are usually called "rogue-lites" or "roguelike-likes".

#### Use of pseudo-randomness and procedural generation

This is done to increase replayability: the dungeons (or levels alike) are generated procedurally, with a tinge of randomness added to them. Joining procedural generation and pseudo-randomness is better than simple pseudo-randomness, since the rules applied will make the level beatable without special equipment, as well as lead to more aesthetically pleasing levels overall.

#### Permadeath

In the great majority of roguelike games, the death of a character is permanent. When a character dies, the player will have to begin a new "run": the levels will be generated anew and the available loot will change too.

Usually permadeath is joined with an erasure of the savefile connected to the "failed run", this avoids so-called "save-scumming": a practice where players would load back their savefile repeatedly to achieve better results (which is usually considered akin to cheating, in the roguelike field).

Another way to stop "save-scumming" is deleting the savefile when loading it, so when you save the only thing you can do to keep your savefile is exiting the game.

Permadeath makes the "save game" functionality more of a "suspension of the gameplay" instead of giving the player a recoverable state they can limitlessly return to.

#### Turn-based Gameplay

Like tabletop games, the gameplay of roguelikes is usually turn-based: this allows the player to take as much time as needed to take a decision.

#### Lack of mode-based gameplay

Roguelikes don't have a real concept of "progression": they allow you to do anything from the get-go, without blocking any action just because you're at a certain point in the game.

#### Multiple ways to accomplish (or fail!) a task

Roguelikes usually allow you to complete a task in many different ways, so many in fact that it seems the developers thought of everything. Let's take for example a locked door, a roguelike game would give you many options:

- Find the key or trigger to open such door;
- Lockpick it;
- Burn it down;
- Find a way around it;
- Kick it down;
- ...

This also means that you have to be careful with your actions: if a weapon freezes entities when it touches their flesh, you better have a pair of gloves handy.

#### Resource Management is key

Resource Management in roguelike games is vital: usually they feature a hunger mechanic, as well as healing items, weapons and various loot that the player must sort through to be able to survive. The player will be forced to leave some loot on the floor of the dungeon, or choose between a known weapon and something unknown that may be weaker or "cursed".

#### Peace was never an option

Most roguelike games are based on hack and slash mechanics, where your main goal is killing monsters. In this kind of games, "peaceful options" don't exist (although they may exist, in a somewhat temporary fashion, to put leverage on some stealth mechanics - like getting a better weapon to kill a powerful enemy by first sneaking around them).

#### Dealing with the unknown

Roguelike games are heavily based on the concept of "unknown": you need to explore an unknown place, finding loot which powers are unknown and should be identified. Magical items change with every run, and give just vague descriptions (like "a red potion") which may heal in one run and kill you in another.

Furthermore items can be subject to change, acquiring or losing traits due to environmental alterations or player modification.

Tips and Tricks
----------------

### General Purpose

#### Make that last Health Point count

Players love that rush of adrenaline they get when they escape a difficult situation with just one health point. That "just barely survived" situation can be "helped" by the game itself: some programmers decide to program the last HP in a special way.

Some prefer giving the last health point a value that is higher than the other health points (kind of like a "hidden health reserve"), others instead prefer giving a brief period of invincibility when that last "1HP" threshold is hit.

These small devices allow you to give players more of those "near death" experiences that can give players that confidence boost to keep them playing through a hard stage, while at the same time, reducing the chance that they will rage-quit.

::::: trivia :::::
This was implemented in both DOOM and Assassin's creed, where the last portion of health had more "hit points".

In Bioshock when you take your last point of damage, you get about 1 or 2 seconds of invulnerability.
::::::::::::::::::

#### Avoiding a decision can be a decision itself

An interesting way to make the characters from a game seem more real, is registering the "lack of response" or "lack of action" in the game's AI or dialogue tree.

This means that "ignoring" has consequences, and inaction is in and itself an action of "doing nothing" which should be accounted for, just like ignoring someone in real life can have serious consequence or where someone may prefer to do nothing instead of taking one of many bad decisions.

:::: trivia ::::
This trick is used in the game "Firewatch", where not responding to a dialogue prompt is a noted decision.
::::::::::::::::

#### Telegraphing

Players hate the feeling of injustice that pops out when a boss pulls out a surprise attack, that's why in many games where precise defense movement is required bosses give out signals on the nature of their attack.

This "telegraphing" technique, allows for that "impending danger" feel, while still giving the player the opportunity to take action to counteract such attack.

Telegraphing is a nice way to suggest the player how to avoid screen-filling attacks (which would give the highest amount of "impending danger").

![Example of a telegraphed screen-filling attack in a shooter](./images/game_design/telegraphing.png){width=60%}

Another form of telegraphing is showing where the attacks will come from, using a "charging up animation": this will attract the player's attention towards those spots and help them gauge the next attack.

:::: trivia :::::
A form of telegraphing was used in the Bioshock series: the first shots of an enemy against you always miss, that is used to avoid "out of the blue" situation, which somehow communicates both the presence and position of enemies.
:::::::::::::::::

#### Minigames

Many times underrated, minigames are a really vital part of a great game experience.

Minigames can be a fun diversion from the main game, extending the engagement time, as well as a priceless resource for bigger open-ended games: you can use common "low-level" materials to feed into the minigame to get better materials, weapons or prizes.

This is a win/win situation, you throw away unused materials to get useful tools, materials or cosmetics, also playing into the mechanism that maybe some people will get things wrong and need the "low-level" materials again, further lengthening the engagement of your game.

#### When unlockables are involved, be balanced

When you're creating a game that involves "unlockables" (for instance a roguelike where you unlock more items for the upcoming runs), you should absolutely balance your unlockables in a way that compels the player to unlock them.

If you hide a "negative item" behind an unlockable, the player will actively avoid doing the actions that lead to unlocking such item. This is especially true now, in the age of widespread Wikis.

If you have to implement unlockables you should either:

- **Make the unlocked item a "good item":** this will naturally compel the player to unlock such item to make the subsequent runs easier and more fun and varied;
- **Make the unlocked item a "neutral item" with situational good outcomes:** the player will be less attracted by these items, but the situational good outcomes (we can call them "interactions" or "synergies") can make the player willing to put in the effort to unlock such item;
- **Unlock 2 items at once: a "good" one and a "bad" one:** the player may be less attracted by this "good+bad" combination, but may still be willing to go through with the unlock effort for the sake of the "good item" and also for added variety in their next runs;
- **Make the unlocked item a "bad item that can become good":** this way the player may be attracted by trying to make a synergy using a "bad item" to cancel its bad effect while keeping the good ones. This allows the player to get engaged in "making builds" for their character.

If you really want to gate a "bad item" behind "a gate", a good idea would make the "bad item" a pre-requisite for unlocking a "very good item". Alternatively, you can unlock the bad item "on the way" to unlocking a "good item": for instance you can make "beat the first boss 5 times" a requirement for the bad item to be unlocked, while "beat the second boss 10 times" could be a requirement for the "good item" to be spawned.

::: tip :::
Remember: you should always account for wikis, some people think that wikis "ruin the surprise" of the game, while others use wikis just out of curiosity, some again instead use wikis as a "guide" to make the game easier or organize their strategy better.
:::::::::::

### Shooters {#gd_shooters}

#### Make the bullets stand out

One of the most annoying things that can happen when you're running-and-gunning your way through a level is being hit by a not-so-visible bullet.

Your own bullets, as well as (and most importantly!) the enemies' should stand out from the background and the other sprites, so that the player can see and avoid them.

Some people may want to ask why your own bullets should stand out too, the answer is: so you can easily aim for your targets.

[^chests]: 32x32 Chests attribution: Bonsaiheldin ([http://nora.la](http://nora.la)), hosted at [opengameart](https://opengameart.org/content/treasure-chests-32x32)

[^icetiles]: Simple SVG ice platformer tiles, listed as "Public Domain (CC0)" at [OpenGameArt.org](https://opengameart.org/content/simple-svg-ice-platformer-tiles-16x16-16x96-96x16)

[^undeadfossil]: Fossil (Undead) RPG Enemy Sprites attribution: Stephen Challener (Redshrike), hosted by [OpenGameArt.org](https://opengameart.org/content/fossil-undead-rpg-enemy-sprites)

### RPGs

#### Grinding and farming

When it comes to games part of the RPG genre, two words must be in your dictionary: **grinding** and **farming**, both as a player and as a game developer.

Sometimes used as synonyms and similar in execution, these terms are actually different and have different objectives. Let's see how.

##### Grinding

Much like "grinding an axe", grinding in RPGs entails cleaning areas from enemies repeatedly (either by re-playing missions or just doing random encounters) with the objective of earning "experience points", thus making yourself stronger.

Grinding is somewhat a "self-leveling game design hinge", allowing you to have some leeway when designing the difficulty of your levels: if a player likes having an easier time, they will "grind themselves" to a higher level; if instead they prefer a challenge, they will power through the "easier parts" until they find the challenge they seek (due to being probably "underleveled").

We can also use "designed grinding", (as well as "level gates", where you need to have a certain amount of experience to continue) to pace our game and eventually even lengthen the experience a bit.

:::: tip ::::
When designing your levels and "designing your grind", you need to be mindful of your target audience.

Some cultures are used to (and enjoy) a higher amount of grinding than others, so too low of an amount may feel unsatisfactory to them, while an amount too high may be frustrating.
::::

You should also be very careful on "forcing grinding" on your players: players like having choice and really dislike having anything forced on them, and this can change with your target audience.

:::: trivia ::::
Super Hydlide for the Sega Genesis/MegaDrive is one of the games that had its experience requirements tailored to the tastes of the market it was targeted to.

Considering the Japan release as the baseline, the US release sees its experience requirements halved.
::::::::::::::::

##### Farming

Farming entails the same actions as grinding, but here we are using enemies as "farming animals", the objective is obtaining a certain amount of materials to obtain a weapon or item.

The most important aspect of "designing farming" is definitely reward the player for their farming: if a "special item" requires a lot of materials (and thus a lot of farming), such item should be worth the effort, or the player will feel cheated out of their time, effort and materials.

:::: note ::::
You should be mindful that some players will exploit some of your more complex mechanics to be able to farm for items and currency faster. This happened in the game "The Witcher III", where players used to kill cows and then meditate to make such cows respawn.
::::::::::::::

:::: trivia ::::
In the game "The Witcher III", precisely in patch 1.05, a mechanic to prevent such exploitation was introduced. The "Bovine Defence Force Initiative" consisted of a respawning moderately-leveled monster (called "Chort") that attacks the player, thus making the farming very dangerous for low level players.

On the flip side, higher level players could exploit this endlessly respawning monster to gain Chort Hides, which are worth more currency than Cow Hides. This was patched by making only one Chort spawn.
::::::::::::::::

{{placeholder}}

<!-- TODO: Finish talking about farming -->

#### Leveling Curves

When it comes to RPGs there are different ways to "design the leveling curve" in a game, depending on how you prefer designing your game.

##### "Exponential" curve

One of the most known ways to shape your leveling curve is making each further "skill level" require more experience points than the previous one, for example:

| Level | Experience Points |
| :---: | :---------------: |
| 1     | 5.000             |
| 2     | 10.000            |
| 3     | 20.000            |
| 4     | 40.000            |
| ...   | ...               |

Table: An example of exponential level curve

This type of leveling curve entails that each newer (harder) enemy gives out a higher amount of experience (not necessarily the quantity to make the experience feel "linear").

The exponential nature of this level progression allows the "push" players towards harder enemies, since grinding lower-tier enemies becomes less and less efficient the more your level increases.

##### "Level-based" experience rewards

As an addition or an alternative to the exponential curve, some games try to "push" the players more towards harder enemies by scaling (down) their experience points with your level.

This method allows you to further shape the curve not only to try and prevent mindless grind of low-tier enemies, but also by rewarding challenging harder enemies.

This method can be implemented in many ways, one of them could be assigning a level to the enemy itself and then scale the experience rewarded by the difference between the player level and the enemy level.

| Player Level - Enemy Level | Experience Reward |
| :------------------------: | :---------------: |
| < -3 (very underleveled)   | 35.000            |
| -2 (moderately underlevel) | 20.000            |
| -1 (slightly underlevel)   | 10.000            |
| 0 (ideal)                  | 5.000             |
| +1 (slightly overlevel)    | 2.500             |
| +2 (moderately overlevel)  | 800               |
| > +3                       | 1                 |

Table: An example of "level-based" experience rewards

Perceived Fairness
------------------

### You don't need precise collision detection

Here we go, spouting bold claims again. Here's a bolder one: you don't **want** precise collision detection. It's slower, harder to implement and most of all, the player may get annoyed at it.

In the heat of a gaming session, with all the action going on, the player may become a bit "blind" to small things: this means that they will perceive as arbitrary anything that is not "evident".

Let's take this arrow vs "generic jumping man" example:

![A pixel perfect detection would trigger in this case](./images/game_design/collision_1.png){width=40%}

With still images, things are obvious: that's a collision, it's on the tip of the character's toe, but it's a hit. It may be 1 or 2 pixels (it's actually 3), but it is still a hit.

Now let's imagine the image moving: the arrow darting from left to right while the character is ascending, it looks like a near-hit, the player will appreciate the adrenaline rush of a near-death. If we go and declare that as a "hit", the player won't understand why, they will say "that's unfair, it just missed me". Our collision detection was **too precise**.

![A smaller hitbox may save the player some frustration](./images/game_design/collision_2.png){width=40%}

Sometimes we just need a hitbox that is small enough to avoid these "looks-like-a-miss" incidents: the instances where the collision detection triggers are evident and the player will appreciate the sensation given by the near-deaths.

Obviously you need to strike a balance, if the hitbox is so small that evident hits are counted as misses (with few exceptions, like [bullet hells](#bullethell)) it will break the immersion.

### Immediate dangers should be well visible

One of the biggest frustrations a player can encounter is definitely being damaged (or killed) by a hidden object.

To avoid these "unfair shots", we should draw immediate dangers "late" in the drawing phase of the game loop, but there is an issue: we don't want to break the immersion.

Let's take the following example: a bomb gets spawned just behind a chest by an enemy, and our character is dangerously close to it.

![A bomb spawned behind a chest, drawn in a realistic order](./images/game_design/dangers_1.svg){width=30%}

If the bomb is "behind" the chest, we can't suddenly make it pop "in front of" the chest, that would definitely ruin the immersion, also it may end up messing with the forced perspective that some 2D games use.

![Moving the bomb in front of the chest may ruin immersion](./images/game_design/dangers_2.svg){width=30%}

As you can see, even though the bomb has a shadow, it looks like the bomb and the shadow are "floating mid-air", thus ruining immersion.

Different games implement different solutions to the problem, some prefer highlighting the danger with an outline drawn over everything, something like the following:

![Highlighting the hidden part of a danger can be useful](./images/game_design/dangers_3.svg){width=30%}

Other instead prefer making the "foreground objects" semi-transparent, so that the player can see what lies behind.

![Making objects transparent is a solution too](./images/game_design/dangers_4.svg){width=30%}

This solution is usually applied when the player themselves are behind the obstacle, giving a more "interactive" and "less confusing" feel to the entire game.

Miscellaneous
-------------

This section denotes some various things that don't really fit in "tips and tricks" but are still related to game design.

### You cannot use the "Red Cross" in games

::: note :::
What follows **is not legal advice**. I am not a lawyer.

If you want to know more (as in quantity and quality of information), contact your favourite lawyer.
:::::::::::::::

When you are developing a game, you will be tempted to use the famous "red cross" symbol on your health packs or health-related items.

**Don't do that**

The red cross symbol (a red cross over white background) is not in the public domain, but it's actually a symbol governed by the ICRC (International Committee of the Red Cross) and you may get in legal trouble for misusing it.

Halo and Doom changed their health packs symbol, from the red cross to a "red H" and a "pill" respectively.

The ICRC enforcement of this rule is inconsistent, but it would technically be a violation of the First Geneva Convention, chapter VII, articles 44 and 53.

Also states themselves (like Canada) tend to have rules of law regulating the use of the symbol in more detail.

### Auto-saving

Some people may consider auto-saving a simple "quality of life improvement", but it can also save the players a lot of frustration in case your game crashes: trust me, no matter how good your programming is, your game will crash (it may be a buggy graphics driver, an edge case that hits 0.0001% of the time or just bad luck).

If possible, you should provide the player with both an auto-save feature and a "manual save" one, this way the player can save where they want but also have a back-up just in case.

To implement an auto-saving feature, we need a slot to auto-save into, so we can choose one of two ways:

- **Choosing the save slot when starting a new game:** this means that the auto-save feature will auto-save and overwrite the selected save slot at every major event, which may be not desired. This is where the manual saving feature comes handy: allowing the player to save manually will also allow them to create a backup savefile.
- **Dedicated "auto-save" slot:** this leaves the manual saving feature intact, but also adds a "special saving slot" the player can't save onto. This slot is dedicated to the most recent auto-save (regardless of the save slot we load from).

### Feedback is important

It is extremely important to add feedback to actions, such as hits: a good visual feedback and the right sound can make all the difference in the world for your game.

The most common visual reaction to a hit is lighting up (by adding a white overlay) the sprite that got hit: this way it is really evident that a hit happened.

The visual feedback should also mirror the effectiveness of the hit too. An explosive weapon should do tons more damage than a single bullet: if this doesn't happen the weapons will feel unbalanced and just badly designed.
