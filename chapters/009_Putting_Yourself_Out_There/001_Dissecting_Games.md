{{pagebreak}}

Dissecting games: three study cases
===================================

:::::: {.epigraph author="Hal Sparks"}
I'm a writer, so I like dissecting things.
::::::

In this section we will talk about three games, one bad and two good, and study their decisions closely, how a bad game inconveniences the player or how a good one reduces the space between the player's thought and the action on the screen.

This is not intended as a section to diss on a bad game or sing the praises of a good one, it is meant to be a study on how bad decisions pile up into what is universally recognized as a "bad game", while there are so many good decisions that need to be taken to make a "good game".

:::: note ::::
This section will contain spoilers of the games. This is necessary to understand them completely.
::::::::::::::

A bad game: Hoshi wo miru hito
-------------------------------

### Introduction

"Hoshi wo miru hito" (roughly translated to "stargazers") is a Japanese turn-based RPG for the NES/Famicom, set in a future where everyone has "Extra Sensory Perception" (ESP) and where a supercomputer has enslaved humanity via brainwashing.

The story may sound thrilling but the game is not. Let's see why this game is also known as "Densetsu no Kusoge" (I will leave finding the meaning to the reader).

### Balance Issues

#### You can't beat starter enemies

At the beginning of the game, you deal only 1 point of damage per attack, way less than the health of the common starter enemy. This, together with your starting health points, make the starter enemies unbeatable most of the times. Speedrunners usually end up resetting in case they get an enemy encounter at the beginning of their run.

#### The Damage Sponge

One of the characters, called "Misa", is the only character that is able to walk on damage tiles without getting hurt. There is no explanation on the reason behind it.

This means that you have to die multiple times before finding out that only one of the four characters in your party is able to cross certain floor tiles, that may be no different than the other tiles.

#### You can't run away from battles, but enemies can

In this RPG you lack the option to run away from battles.

Enemies instead have a chance to run away from battle when their health points drop below 25% of their original health. Talk about fairness.

The "escape" option is instead hidden behind the "teleport" spell that you acquire after leveling up, in addition such spell is really weird in its way of working.

After selecting the "teleport" spell, you select a team mate to target such spell to, the spell can either succeed or fail:

- If the spell succeeds, the selected team member escapes the battle, while the others continue fighting for the turns that follow;
- If the spell fails, the whole team gets ejected (read "escape") from the battle.

This means that the teleport spell is more beneficial (4 times faster) when it fails than when it succeeds.

:::: note ::::
To be more precise, if the teleport spell succeeds and you manage to get each character (one by one) out of the battle, you keep your position on the current map. If instead it fails, your entire party will be kicked from the battle and you will find yourself at the beginning of the map. In practical terms, that doesn't make much of a difference, because you will probably need to get to the nearest healer anyway.
::::::::::::::

#### Statistics

There are some statistics that make sense in other RPGs, but do not in this game.

For instance the "defense" statistic scales so poorly that you barely notice its effect in this game.

In other games the "speed" statistic is tied to the order of attack (from the quickest to slowest character), but in this game the order is always "player's team" first, and "enemy team" after.

In conclusion, in "Hoshi wo miru hito", defense is effectively useless while speed is not even implemented.

### Bad design choices

#### You get dropped in the middle of nowhere

In the NES era, it was common thing to have the story written in the manual. To save space on the cartridge, the beginning story elements were reduced to a minimum or completely removed, but in most games you still had a sense of where to go.

In this game, you just get dropped in the middle of nowhere, with no direction whatsoever. And you don't have the "Legend Of Zelda" style of exploration, since any enemy can make minced meat of you.

As a comparison, Dragon Quest, a game from the same period, had at least a hearing with the king to still introduce you into the story.

#### The starting town is invisible

The previous point is not really true, you actually start near a town, but such town is invisible.

The game makes a really lousy attempt to justify the fact that the town is invisible, but such explanation falls absolutely flat.

This just adds to the confusion of the story, as well as the lack of direction given to the player which can result in frustration and abandoning the game.

#### The Jump Ability

At level 1, you acquire a "jump ability", that allows you to jump over certain tiles, like rivers. The issue is that such tiles are not distinguishable in any way from all the other tiles.

So you will find yourself mashing your main character's body against various tiles, trying to find which ones you can skip with your jump ability, and probably die in the process by finding an unrecognizable damage tile.

#### Items are invisible

All items in the game are invisible, including all plot-crucial and revive items. The only thing telling you that you found an item, is a "beep" sound when you collect them.

This further piles up with the lack of direction the player faces in this game since the beginning. While it's understandable that the limited size (and therefore duration) of NES/Famicom games kind of forced the developers' hands into making harder games (to make them last longer), but introducing confusing of flat-out unfair mechanics is just bad design.

#### Item management

Usually when you buy a new weapon inside an RPG, you get to un-equip the old weapon and substitute it with the new one, then eventually sell the old one to recover some currency. This gives the game's challenge new dimensions: item management and a simple economy system.

Well, this game instead lacks any kind of item management: every time you buy a new weapon, the old one will be automatically discarded. You cannot sell old weapons, and the auto-discard removes the possibility of trying a new weapon and in case go back to the old one.

And you cannot un-equip items and weapons.

#### Buying Weapons makes you weaker

When unarmed, from level 1 onward, the fight option lets you deal a damage equal to a random number between 0 and 4 (bounds included), regardless of the enemy defense stat, which is a real low amount of attack power.

When armed, the enemies defense values are taken into account instead, which means that most of the time, the boosted attack power given by the weapon doesn't overcome the enemies defense enough to make using weapons an advantage.

In few words: buying weapons makes you weaker.

And, as stated before, you cannot un-equip weapons, so your game session is probably ruined.

#### Enemy Abilities

Many enemies have an ability which is essentially a permanent, non curable in battle, paralysis + poison combo that will make your battle really hard and frustrating. That means that you will lose all the turns of the character that has been hit with such status effect.

And in case all your party members are hit with such status effect, you don't game over immediately, instead you will keep losing turns while the enemies slowly chip away at your party's health until you eventually game over.

Such effect lasts outside of battle too, so every step you take the affected party members will lose health until you see a healer.

#### You can soft lock yourself

In the vast majority of games, keycards are usually a permanent item that can be reused after finding it. In other games instead doors opened with keycards stay open for the rest of the game.

In this game, keycards have to be bought for quite the price, and disappear on use, and there is a serious chance that you softlock yourself somewhere if you don't buy enough.

### Confusing Choices

#### Starting level for characters

This may be minor, but your characters start at level 0. Simply confusing and weird from a player standpoint.

As a programmer I find it quite amusing, though.

#### Slow overworld movement

The movement in the overworld is really slow, at around 1 tile every 2 seconds. This is really confusing, since the NES/Famicom is definitely capable of higher performance.

This is not due to lag or performance issues, it is a conscious decision to make the characters walk so slowly.

#### Exiting a dungeon or a town

Every time you exit a town or a dungeon, you won't find yourself at the entrance or exit of such place (like you'd expect), but instead you will find yourself at the default spawn of the world you're in.

So you may find yourself at the beginning of the game or in some not easily recognizable point of the map.

#### The Health Points UI

In the battle UI, the health of your team members is shown on top of their pictures, as an overlay.

Given the size of the font and the size of the pictures, only 4 digits fit. Given the game's health scaling, there is a serious chance that you get your health points to 5 digits.

The solution adopted was to drop the last digit of the health counter in all cases (even if your maximum health has less than 5 digits): so if you see "15" your health is actually between "150" and "159".

Also for some reason, if your health is lower than 10 points, your health shows as 0 (my speculation is that is would be written as "00" to "09").

### Inconveniencing the player

#### The battle menu order

In the great majority of turn-based RPGs, the options are shown in the following order:

1. Fight
2. Magic (ESP)
3. Items
4. Escape

This is done in order, from most used (fight) to least used (escape).

In "Hoshi wo miru hito", the menu order presents the ESP option as the first option, selected by default, so most of the time you will have to move your cursor to the "fight" option and select it. This compounds with another problem exposed below.

#### Every menu is a committal

There is no "back" option in any menu, this means that every menu is a committal and you can't back off from any decision, even accidental ones.

That means that if you accidentally select the ESP option in battle and you don't have enough energy/mana to execute any attack, you will end up losing a turn.

If you select the wrong ingredient to make a potion, you most probably will have to waste that ingredient.

#### Password saves

In the NES/Famicom era, games that made use of battery-backed RAM modules to save game progress were rare. This means that the most used save method was using "passwords": a jumble of letters (and eventually numbers and symbols) that needed to be written down precisely, or you wouldn't be able to restore your progress.

This game's passwords are **massive** and use a mix of katakana Japanese characters and English alphabet, (while the rest of the game uses hiragana characters), which can be confusing.

Also passwords don't really save your progress "as is": your levels are saved in multiples of 4 (so if you're level 6, you will restore your progress but be level 4) and money is saved in multiples of 255 (if you have 3000 gold, you will restore your progress but have $255 \cdot 11=2805$ gold)

#### Each character has their own money stash

In most RPGs that feature a party, there is a shared pool of money that is used for all expenses, this may not be "realistic" but it's a good enough approximation that has a major upside: it is practical.

This game instead inconveniences the player further by giving each party member their own separated money stash. This is realistic and sometimes used in more modern RPGs, but it is not practical: every time you need to purchase potions used by the whole party (remember: there is no item management) you will have to switch characters or you'll find yourself running out of money.

### Bugs and glitches

#### Moonwalking and save warping

This game doesn't interpret inputs as well as it should, so if you press the up and down buttons at the same time, you will find yourself "moonwalking".

Besides the perceived coolness of such move, moonwalking will allow you to go through obstacles, and eventually corrupt the graphics of the tilemap (like loading the right side of the map on the left side of the screen).

This is due to the game checking one direction for wall collisions, but moving the character in the opposite direction.

Pressing up and down at the same time on a controller is not possible, due to the fact that the NES/Famicom D-Pad does not have separated buttons, but if you connect any accessory that allows you to connect up to 4 controllers, the game won't be able to distinguish between the inputs from Controller 1 and the ones Controller 3.

A side effect of moonwalking, used in speedrunning is "Save Warping", you are able to manipulate the tilemap and your position via moonwalking, then save and *voilà* you will be warped to another point of the map.

#### The final maze

The final maze is divided in multiple floors, and is the greatest proof of how rushed this game was.

In the first floor of this maze, which is supposed to be really hard, no encounter tiles have been programmed: this means you won't have to fight anything on this floor. Also no "wall collision" was programmed either, so you can go through the maze walls with the same ease you walk on the floor.

In the other maze floors, encounter tiles were programmed, but still no wall collision was implemented, and since you can't encounter anything on walls, you can just minimize your encounter chance by taking a stroll inside the maze's walls.

#### The endings

This game, very ambitiously I shall say, features multiple endings. Towards the end you have to take a very hard decision:

- Join your enemy and leave humanity and live peacefully
- Leave the disputed territory and let the enemy live in peace
- Fight to gain control over the disputed territory

This can result in four different endings, which is really ambitious for a NES/Famicom game. If only the final boss fight was implemented...

If you choose to fight, you will automatically lose the battle and the game will end with a "bad ending".

### Conclusions

"Hoshi wo miru hito" is a product of its situation, rumors state that this game was programmed by only one person, and rushed beyond belief so it could compete with *Dragon Quest* in some way. For the sake of fairness, I will assume that this game was made by a team.

The game has interesting ideas for its time: a cyberpunk theme, Extra-sensory powers, the character sprites "grow up" as they gain levels, the enemy sprites are artistically well-done, ... but the execution is littered with problems, obstacles to the player, bad design decisions and bugs.

It seems that the developers were not familiar with the NES/Famicom architecture, game designers weren't really familiar with game design concepts and play testers were completely nonexistent.

Even though this game has earned the status of "legendary bad game" (not a literal translation of "Densetsu no Kusoge"), "Hoshi wo miru hito" has gained a cult following that is really devoted, to the point that a hacker took upon themselves the daunting task of patching the game and redraw the graphics, as well as rebalancing the weapons and fix the walking speed.

There is even a "remake" called "STARGAZER" for windows.


The first good game - VVVVVV: Slim story and essential gameplay
----------------------------------------------------------------

VVVVVV is a 2D platformer created by Terry Cavanagh that features essential gameplay mechanics, a slim story that gives the player a reason to explore the game world and get to the end, as well as a satisfying level of challenge. Let's see what makes this a good game.

### A Slim story that holds up great

VVVVVV's story is as essential as it gets: you're in a space ship, you run into some trouble and try to teleport out of the ship. Now you and your crew are scattered in a new dimension. Your mission is to rescue your crew and explore the new dimension you're in.

This is the story, it gives you enough of a reason to move from one level to the other, without being too burdensome. After you finish the game, you are free to explore the entire "dimension VVVVVV", searching shiny trinkets to unlock a final secret and learn more about the dimension you're in.

### Essential gameplay: easy to learn, hard to master

In VVVVVV you can move left and right, but you cannot jump: you can only flip gravity. This, combined with the plentiful quantity of spikes, makes for a game that is easy to learn (it's just 3 buttons) but hard to master (you will find yourself dying over and over on the same screen).

The gameplay is so simple and well-implemented that it's really hard to get mad at the game: if you die and respawn, you know it's your fault. The great majority of time, it is evident. When you learn what "not to do", the rest becomes a lot easier. Enough so that you don't get stuck in a screen for more than a few minutes.

Checkpoints are plentiful and well distributed, so you won't end up going back too far if you die.

### Diversified challenges

Each zone of the game (there are 6 in total, plus 2 intermissions and the finale) features the tight gameplay explained above, and sometimes add a new gimmick to the mix.

Let's explore those new gimmicks quickly:

- In the "Warp Zone", when you exit the screen on one side, you will re-enter on the opposite side: this means that if you fall on the bottom, you'll come out the top. This goes on until you reach the exit of the screen;
- In "the lab", there are lines that make you invert your gravity when touched. This can end up being a bit confusing at first, until you get used to the mechanic. After that you'll be bouncing around with no issue;
- In "the tower" the level automatically scrolls vertically: you can't go too slow or too fast, or spikes will come out of the top/bottom of the screen to kill you;
- In "intermission 1" you are followed by a crew mate: if you're standing on the ground, they will walk towards you. If you're on the ceiling, they will stay still. This introduces a new layer of difficulty and management;
- In "intermission 2" (also known as "the gravitron") you are kind-of-followed by a crew mate, but in reality you're playing alone. You will have to survive 60 seconds (with a checkpoint every 5 seconds) between two gravity-inverting lines, while items are shot at you.

In the finale, you'll have to put everything you have learned to the test ("warping", "bouncing", dealing with auto-scrolling levels, ...) to save yourself from being a prisoner of dimension VVVVVV.

### Graphics

The graphics try to imitate the Commodore 64 (there is even a fake C64-style loading screen!); but they don't give up special effects like flashes, animated sprites, animated tiles, screen shaking, and fully moving backgrounds.

Even though the graphics are superior to what a Commodore 64 would be able to output, the special effects used still fit the chosen style and never really feel "out of place".

### Amazing soundtrack

The soundtrack in VVVVVV is definitely one of the game's highlights: the chiptune-style songs that characterize every zone are catchy and so memorable that you will find yourself humming the tunes from time to time.

Each tune fits the zone it's used in, and kind of "tells a story" of its own: from "Passion for exploring" (the overworld theme) to "Predestined Fate" (used in intermissions and, in a remixed fashion, in the finale).

### Accessibility Settings

VVVVVV is as accessible as it gets: there are a ton of accessibility features crammed in such a small game. Let's take a look at them.

- **Invulnerability** in case the challenge is too much, or you don't want to feel challenged all the time. This also helps people who want to enjoy the game, but can't due to mobility problems;
- **Slowdown mode** some people with mobility issues or slower reflexes may benefit from playing the game at 75, 50 or even 25\% speed;
- **No screen flashing or shaking** some people with photosensitivity may have huge issues with the flashing and screen shaking, there are options to disable these effects and make the game much safer for those people. It also helps if flashing and screen shaking just annoy you or simply give you headaches;
- **No animated backgrounds** this may help with visual clarity or if the movement in the background gives you issues, like headaches.

### Post-endgame Modes

After finishing the game, VVVVVV still offers some challenges. If you haven't collected all the trinkets, that is a good start, since they're hard to get and will reveal a secret back at the ship.

VVVVVV offers other game modes too, after you finish the main story:

- Time trials: you can replay any level, but you have to finish it under a certain time limit.
- Flip mode: the whole game is flipped vertically;
- No death mode: the entire game must be played without dying, you cannot save, there are no checkpoints and your companions in intermissions cannot die either;
- Intermissions mode: replay all completed intermissions, you can choose your companion too.

### User-generated content

If the main story didn't satisfy you enough, VVVVVV features a level editor. The game already includes some selected user levels you can play, each with its own story and mechanics.

This also means that you can access the level editor yourself and create your own adventure from scratch, featuring the mechanics and characters of the original game.

### "Speedrunnability"

Being an exceptionally difficult game, as well as the simple controls, the game has attracted an active and passionate speedrun community. The first playthroughs can take up to a couple hours, but an average speedrun can take less than 20 minutes!

The barrier of entry (usually for "glitchless" speedruns) is very low: you just need to go fast and die as little as possible. There aren't too many tricks to be learned, and those are usually quite simple, with very few exceptions.

### Characters are memorable, even if you don't see them a lot

Even though the characters are not a continuous presence in the game, each of them is memorable: they all have very different colors, and different characters.

For instance: Vermillion is adventurous and always excited about exploring, after rescuing him, you will find him here and there in the overworld, sometimes even in the zones where other crew mates are stuck.

Victoria is a bit of a crybaby, very emotional and gets depressed very easily, she's always feeling blue (and blue is her color too) and sniffles a lot when talking.

Verdigris is technical, being the ship's technician (you will find him working on the ship's antenna after rescue). Professor Vitellary is analytical and curious (if you bring him into an intermission, he will express marvel at what's happening).

### Conclusion

VVVVVV is a small game (as I said, it can be completed in less than 20 minutes if you're quick), but it gives a lot of options for everyone. Lots of accessibility, replayability and "speedrunnability". Custom levels and the level editor are the cherry on top of a game that is feature-complete and fun to play.

Another good game - Undertale: A masterclass in storytelling
----------------------------------------------------------------

Undertale is an RPG game created by Toby Fox that features some unique mechanics and masterful story telling.

We'll take a deeper dive into the game immediately!

### The power of choice

The game features a huge innovation in the field of RPGs: you can run the entire game without killing anything. In fact you are encouraged from the beginning to do so.

This innovation is not forced onto you though, you can play it as any other RPG out there (but you will miss a lot of the game, more on that later).

### The game doesn't take itself very seriously (sometimes)

Undertale is a very unique game that doesn't take itself very seriously, there are 4th wall breaks, bad puns, jokes, worse puns and more. This gives the game a very lighthearted tone. That is if you're doing a "pacifist run".

The game becomes more somber the more you lean into a "neutral" or even "genocide" run.

This gives the game a lot of layers and depth, making each "type of playthrough" a different experience.

### All the major characters are very memorable

Each major character is extremely memorable, and will be etched into your memory for the entire game (and probably part of your life too), I can recall practically all of them on the top of my head:

- Toriel, a gentle and motherly monster that loves puns and jokes;
- Sans, a skeleton that loves bad puns and "knows shortcuts" to every place in the game, somewhat lazy (that is explained in a lot more detail in genocide runs);
- Papyrus, Sans's brother. Hates bad puns, loves spaghetti and wants to enter the royal guard. Has high self-esteem;
- Undyne, a brave, brash muscle-for-brains fish girl that has actually a very kind heart;
- Alphys, a pessimist doctor with very low self-esteem, even though she's essentially a genius. Somewhat a nerd too;
- Mettaton, the robot that wants to become a TV superstar,
- Muffet, the leader of the spider bakery,
- Asgore, the "final boss" that you have to face. Regretful of his actions and past;
- Flowey, a yellow flower that can't have feelings (this is explained well in the pacifist run).

### The game continuously surprises the player

The game is able to surprise the player continuously. At a certain point the player realizes that the game is playing by different rules than expected: the game (as a piece of software) and the world become blurred when Asgore breaks the "mercy" option in a pacifist run.

From that point on, the player realizes that the UI, saving, loading are all characteristics of the world itself, and not of the game: each playthrough is treated like a timeline, and each reset is a new timeline (although some characters may have memories from previous playthroughs).

It feels like the world inside the game actually exists.

### Player choices influence the game

Each run can be a bit different than any previous run, but they can all be classified into 4 categories.

- **Neutral runs** these are the runs that entail killing some enemies, but not everyone. The ending of this kind of runs is not satisfactory, and the game loses its meaning. The game will suggest (via its characters) to try a different approach;
- **Pacifist runs** these runs entail not killing anyone, this will bring an extended playthrough and ending, explaining a lot more about the world (and the "meta" nature of the game);
- **Genocide runs** these runs entail killing everything, even the "random encounters", this will bring a different ending and will permanently change the game, even in successive runs (unless you physically delete your save file from the game folder).
- **Partial/Aborted Genocide** these runs entail killing everything, besides one of the main characters. This will bring the player a different ending each time, but none of them will be "good".

### Great (and extensive!!) soundtrack

The soundtrack features over 100 (a hundred!!) tracks, each of them is unique in itself and memorable. Each zone has its own fitting theme, as does each boss (so much so that many of them have different themes for normal and genocide).

It's hard to describe the soundtrack here, so I suggest you to try the game or at least listen to some of the most famous tracks.

### Conclusion

I tried to keep this analysis vague so not to ruin the game too much to the people who didn't play it. The pacifist story is extremely well-written and ties extremely well with the genocide one, together giving two sides of the same world.

The fact that such world is working with the mechanics of a video game is a surprise to the player, who will be a bit confused at the beginning but will soon understand things that may have felt weird before.
