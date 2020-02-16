\null\clearpage

Dissecting games: two study cases
=================================

In this section we will talk about two games, one bad and one good, and study their decisions closely, how a bad game inconveniences the player or how a good one reduces the space between the player's thought and the action on the screen.

This is not intended as a section to diss on a bad game or sing the praises of a good one, it is meant to be a study on how bad decisions pile up into what is universally recognized as a "bad game", while there are so many good decisions that need to be taken to make a "good game".

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

#### Can't run away from battles, but enemies can

In this RPG you lack the option to run away from battles.

Enemies instead have a chance to run away from battle when their health points drop below 25% of their original health. Talk about fairness.

The "escape" option is hidden behind the "teleport" spell that you acquire after leveling up, although such spell is really weird in its way of working.

After selecting the "teleport spell", you select a team mate to target such spell to, the spell can either succeed or fail:

- If the spell succeeds, the selected team mate escapes the battle, while the others continue fighting;
- If the spell fails, the whole team gets ejected (read "escape") from the battle.

This means that the teleport spell is more beneficial when it fails than when it succeeds.

#### Statistics

There are some statistics that make sense in other RPGs, but do not in this game.

For instance the "defense" statistic scales so poorly that you barely notice its effect in this game.

In other games the "speed" statistic is tied to the order of attack (from the quickest to slowest character), but in this game the order is always "player's team" first, and "enemy team" after.

In conclusion, in "Hoshi wo miru hito", defense is effectively useless and speed is not even implemented.

### Bad design choices

#### You get dropped in the middle of nowhere

In the NES era, it was common thing to have the story written in the manual. To save space on the cartridge, the beginning story elements were reduced to a minimum or completely removed, but in most games you still had a sense of where to go.

In this game, you just get dropped in the middle of nowhere, with no direction whatsoever. And you don't have the "Legend Of Zelda" style of exploration, since any enemy can make minced meat of you.

#### The starting town is invisible

The previous point is not really true, you actually start near a town, but such town is invisible.

The game makes a really lousy attempt to justify the fact that the town is invisible, but such explanation falls absolutely flat.

#### Jump Ability

At level 1, you acquire a "jump ability", that allows you to jump certain tiles, like rivers. The issue is that such tiles are not distinguishable in any way from all the other tiles.

So you will find yourself mashing your main character against various tiles, trying to find which ones you can skip with your jump ability, and probably die in the process by finding an unrecognizable damage tile.

#### Items are invisible

All items in the game are invisible, including all plot-crucial and revive items. The only thing telling you that you found an item, is a "beep" sound when you collect them.

#### Item management

Usually when you buy a new weapon inside an RPG, you get to un-equip the old weapon and substitute it with the new one, then eventually sell the old one to recover some currency.

Well, this game lacks any kind of item management: every time you buy a new weapon, the old one will be automatically discarded.

And you cannot un-equip items and weapons.

#### Buying Weapons makes you weaker

When unarmed, at level 1, the fight option lets you deal a damage equal to a random number between 0 and 4 (bounds included), which is a real low amount of attack power.

When armed, the enemies defense values are taken into account instead, which means that most of the time, the boosted attack power doesn't overcome the enemies defense enough to make weapons an advantage.

In few words: buying weapons makes you weaker.

And you cannot un-equip weapons, so your savefile is probably ruined.

#### Enemy Abilities

Many enemies have an ability which is essentially a permanent, non curable in battle, paralysis + poison combo that will make your battle really hard. That means that you will lose all the turns of the character that has been hit with such status effect.

And in case all your party members are hit with that status effect, you don't game over immediately, instead you will keep losing turns while the enemies slowly chip away at your health until you eventually game over.

Such effect lasts outside of battle too, so every step you take the affected party members will lose health until you see a healer.

#### You can soft lock yourself

Keycards are usually a permanent item that can be reused after getting it.

In this game, keycards have to be bought for quite the price, and disappear on use, and there is a serious chance that you softlock yourself somewhere if you don't buy enough.

### Confusing Choices

#### Starting level for characters

This may be minor, but your characters start at level 0.

I have nothing to say about it. Simply confusing and weird.

#### Slow overworld movement

The movement in the overworld is really slow, at around 1 tile every 2 seconds. This is really confusing, since the NES/Famicom is capable of higher performance.

It is not lag or performance issues, it is a conscious decision to make the characters walk so slowly.

#### Exiting a dungeon or a town

Every time you exit a town or a dungeon, you won't find yourself at the entrance or exit of such place (like you'd expect), but instead you will find yourself at the default spawn of the world you're in.

So you may find yourself at the beginning of the game or in some not easily recognizable point of the map.

#### The Health Points UI

In the battle UI, the health of your team members is shown on top of their pictures, as an overlay.

Given the size of the font and the size of the pictures, only 4 digits fit. Given the game's health scaling, there is a serious chance that you get your health points to 5 digits.

The solution adopted was to drop the last digit of the health counter: so if you see "15" your health is actually between "150" and "159".

Also for some reason, if your health is lower than 10 points, your health shows as 10.

### Inconveniencing the player

#### The battle menu order

In the great majority of turn-based RPGs, the options are shown in the following order:

1. Fight
2. Magic (ESP)
3. Items
4. Escape

This is done in order, from most used (fight) to least used (escape).

In "Hoshi wo miru hito", the menu order presents the ESP option as the first option, selected by default. This compounds with another problem exposed below.

#### Every menu is a committal

There is no "back" option in any menu, this means that every menu is a committal and you can't back off from any decision, even accidental ones.

That means that if you accidentally select the ESP option and you don't have enough energy/mana to execute any attack, you will end up losing a turn.

#### Password saves
<!--
Password saves + passwords get you back at the lower multiple of level 4 and money at a multiple of 255
-->
\placeholder

#### Each player has their own money stash

\placeholder

### Bugs and glitches

#### Moonwalking and save warping

\placeholder

#### The final maze

\placeholder

<!-- In the last maze, first floor, no walls or encounter tiles have been programmed, in the other floors, no walls were programmed -->

#### The endings

<!--
The endings (join, leave or fight: but the latter makes you lose automatically)
-->
\placeholder


### Conclusions

<!-- Product of a rushed release to go against Dragon Quest, seems to be made by only one programmer, now has a cult following -->

\placeholder

A good game: Dark Souls
------------------------

\placeholder
