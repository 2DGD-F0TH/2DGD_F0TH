Tips and Tricks
----------------

### General Purpose

These tips and tricks are good for any kind of game: from the simplest platformer to twin stick shooters, to strategy games. These are good starting points to make your game feel more complete and fun to the player.

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
- **Make the unlocked item a "bad item that can become good":** this way the player may be attracted by trying to make a synergy using a "bad item" to cancel its bad effects while keeping the good ones. This allows the player to get engaged in "making builds" for their character.

If you really want put a "good item" behind "a gate", a good idea would make the "bad item" a pre-requisite for unlocking a "very good item". Alternatively, you can unlock the bad item "on the way" to unlocking a "good item": for instance you can make "beat the first boss 5 times" a requirement for the bad item to be unlocked, while "beat the second boss 10 times" could be a requirement for the "good item" to be spawned.

::: tip :::
Remember: you should always account for wikis, some people think that wikis "ruin the surprise" of the game, while others use wikis just out of curiosity, some again instead use wikis as a "guide" to make the game easier or organize their strategy better.
:::::::::::

### Shooters {#gd_shooters}

#### Make the bullets stand out

One of the most annoying things that can happen when you're running-and-gunning your way through a level is being hit by a not-so-visible bullet.

Your own bullets, as well as (and most importantly!) the enemies' should stand out from the background and the other sprites, so that the player can see and avoid them.

Some people may want to ask why your own bullets should stand out too, the answer is: so you can easily aim for your targets and distinguish your own bullets from the enemies'.

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

Considering the Japan release as the baseline, the US release sees its experience requirements halved as well as the given experience doubled.
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

{{extend}}

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

| Player Level - Enemy Level | Experience Variation  | Experience Reward |
| :------------------------: | :-------------------: | :---------------: |
| < -3 (very underleveled)   | + 600%                | 35.000            |
| -2 (moderately underlevel) | + 300%                | 20.000            |
| -1 (slightly underlevel)   | + 100%                | 10.000            |
| 0 (ideal)                  |   0%                  | 5.000             |
| +1 (slightly overlevel)    | - 50%                 | 2.500             |
| +2 (moderately overlevel)  | - 84%                 | 800               |
| > +3                       | Fixed Value           | 1                 |

Table: An example of "level-based" experience rewards

This can be done in many ways, from changing the formula itself to having "hidden status effects" that change the experience output according to the level difference.

#### "Mastering"

An interesting mechanic to add in RPGs is "mastering", as in "mastering the usage of a tool", this mechanic entails having set bonuses after a certain "mission" is completed with a certain tool.

Some examples could be:

- Kill 1000 enemies with a spear $\rightarrow$ 20% more attack power using spears;
- Resist 2000hp of damage while wearing a heavy armor $\rightarrow$ you can wear heavy armor without running speed penalties;
- Kill 50 high-rank demons with a shotgun $\rightarrow$ shotgun reloads twice as fast.

This can add a new level of depth in your game, as well as giving the player more choice over how they want to shape their character: do they want to spend the effort to get a "mastering bonus" or is their time spent better elsewhere?

:::: trivia ::::
*Tales of Berseria* includes a very interesting "mastering" mechanic: once you master a piece of equipment, its bonus "sticks" to the character and is kept even after the piece of equipment is removed. This stimulates the player to try new equipment to "collect bonuses".

On the flip side, if you keep the same piece of equipment, such bonus is doubled. This way the player can decide between a "double bonus" or "enlarging their bonus pool".
::::::::::::::::
