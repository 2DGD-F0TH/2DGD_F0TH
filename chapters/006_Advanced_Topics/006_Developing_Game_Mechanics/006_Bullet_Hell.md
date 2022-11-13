"Bullet Hell" Style Games {#bullethell}
-------------------------

The common definition of a "bullet hell" game is usually the one of a scrolling (usually space-themed) shooter with a very high level of difficulty and lots of enemy bullets on screen (hence the name).

### Bullets

When it comes to this kind of game, it is vital that the enemy bullets are **well visible** (as stated in the [shooters section](#gd_shooters) in the "game design" chapter), this usually means that their color is brighter and has a lot of contrast with the background and the sprites on screen.

![Example of how to better "highlight" bullets](./images/developing_mechanics/bh_highlight_bullets.png){width=50%}

Having "evident" enemy bullets makes the situation easier to assess, even when the situation becomes really chaotic. If you zoom out (or get your reading support farther from your eyes) you can see that the "non-highlighted" bullets (on the left side) tend to "blend in", while the "highlighted" (on the right side) version stay visible.

To highlight bullets, you can use "complementary colors", as shown in the [use contrast to your advantage](#contrast_to_your_advantage) section.

::: tip :::
Bullet visibility is so important that in many games bullets are the last thing to be drawn before the player: this means they're drawn over explosions, other enemies and your own bullets too.

If you let players lose sight of bullets by drawing graphical effects over them, the game will feel unfair.
:::::::::::


### The Ship Hitbox

In the bullet hell genre usually the player ship's (or character of some kind) hitbox is usually much smaller than the visible sprite, this makes the game a little bit "easier than it seems", but at the same time it doesn't mean that the game is easy either.

![An example of a Bullet Hell ship hitbox](./images/developing_mechanics/bh_ship_hitbox.png){width=50%}

In this image, the ship's hitbox is limited to the cockpit, some games prefer some area that could be considered the "ship engine" while others just have a "core" of some sort.

Many games of the genre even make the hitbox a single pixel!

### Screen-clearing bombs

Another mechanic used in bullet hell games are "screen-clearing bombs": these are used to rid the screen of the gigantic number of bullets on it, to give the player some breathing room.

In some games bombs may be also used to destroy small enemies and damage bigger ones. The screen clearing move can happen in many ways: the most common is just making the bullets disappear, but other games prefer turning the "destroyed bullets" into small collectibles that can give the player points.

### Clearing bullets on pattern changes

Some bullet hell games feature multi-phase bosses, where the boss changes attack strategy, and thus their bullet pattern and speed, at certain points of the fight (usually when reaching a certain amount of health left). This may create some issues to the player, since the new bullets may cover all "escape routes" willingly left by the previous bullets, thus making it impossible to not die.

A simple and effective strategy is clearing the screen of the enemy bullets automatically when the boss changes phase (sometimes transforming the bullets into collectibles for score), this will allow for a quick breather to the player, as well as a somewhat smooth transition to the new phase.

### Find other chances to clear some bullets

Some games find creative ways to clean up a screen cluttered with bullets: for instance some bullets can turn into collectibles when a pickup is touched by the ship.

::: trivia :::
In ZenoDyne R powerups are real, "physical" objects, and as such they block incoming bullets, so they can be strategically used as a "shield", and then pick them up at the last second.
::::::::::::::

Some games like to clear the screen (without giving out collectibles) at the beginning of a boss fight, to give a "clean slate" to start the boss with.

### Turn enemy bullets into collectibles at the end of a boss fight

An interesting form of bonus that is often present in bullet hell games is turning all the boss's bullets on screen into collectibles at the end of a boss fight.

Since this genre of game gets progressively harder the more bullets are on screen, this small trick rewards players for being good at dodging, while players who used screen-clearing bombs will have a smaller bonus.

### The "Chain Meter"

This is a mechanic used in many bullet hell games: the chain meter is a meter that gains value according to the number of enemies you kill in a certain amount of time and giving a score multiplier according to it.

This meter will automatically discharge with time, making it hard to keep up a high score multiplier, adding challenge to the player and rewarding them for being good at destroying enemies in large numbers fast.

Usually the meter has 5 levels, starting from level 1, when the meter is full, the meter "gains a level" and a score multiplier is applied accordingly. For instance we can have:

- Level 1: 1x multiplier (normal score)
- Level 2: 2x multiplier (double score for each killed enemy)
- ...
- Level 5: 5x multiplier

::: tip :::
You can code the "discharge" so it is faster at higher levels. This will bring even more challenge at keeping the level high.
:::::::::::

When the player dies, the counter gets completely emptied and thus the multiplier gets reset to 1x.

::: tip :::
Alternatively, you can halve the level of the meter on player's death.
:::::::::::

### Managing the player's death

It is very common in the "bullet hell" genre to punish the player's death with a strong cut at the ship's power.

This has a problem: a player dying may spiral into a fully-fledged game over because the ship is now extremely underpowered compared to the stage the player died in.

A solution often used in this genre of games is having a dying player's ship have a random chance of releasing a random number of powerups and bomb pickups on death, thus allowing the now-weakened player to "regain some strength" and continue their game.

### The Enemy AI

Probably the hardest part to develop in a "bullet hell" style game is the enemy AI and how to make the enemy bullets form a pattern that is hard but not impossible to dodge.

Since the gameplay is very hard to balance, this genre seldom sees a "procedural game" (an exception that comes to my mind is "Task force Kampas", which features procedural levels, but handmade bosses).

A way to make the game feel more fair is programming the AI so it doesn't shoot "on the way out" of the screen. Each enemy essentially has 3 phases in its AI:

1. Enter the screen
2. Fight (usually by shooting a single pattern or a continuous stream of the same pattern)
3. Exit the screen (or die)

When the enemy exits the screen, it should stop shooting and just orderly leave.

::: tip :::
If your game features enemy turrets, they should stop shooting when they are behind the player's ship: the player is already busy enough handling shots from the front. Shooting from behind makes the game unfair.
:::::::::::

### Be fair to the player, but also to the computer

The title may be a bit awkward: how can you "be fair to a computer"?

Computers don't have feelings, but players do. And letting the players kill the enemy before the AI activates takes away all the challenge from the game itself: the enemies become cannon fodder when the player's weapons have enough "power" to instantly kill most of the enemy forces.

So to apply this, you should probably make the enemies invincible until they're fully on screen: this way the player sees them and doesn't kill them beyond the top of the game area.

### Inertia

Control is everything in a game where a pixel can be the difference between life and death of your ship/character. This means that heavy inertia does not play well with the "bullet hell" genre.

This also doesn't mean that you can't apply any, just be careful and don't go overboard. When a player dies because their ship went too far due to inertia, they will get mad at the game, and by transitive property, at the devs.

### Some examples

There are games that make the most of the "bullet hell" mechanics to give player more challenge, or risk/reward choice.

One game is "Touhou", which has a "grazing" mechanic: if a bullet slightly grazes (but does not hit) your hitbox, you will see some sparks and get a bonus in points.

Another title that makes the most of giving the player a "risk vs reward" choice is Ikaruga, with it's "polarity" mechanic. Your ship has two sides: black and white, each side is able to absorb (and so is also immune) to the bullets of the same color, but also does more damage to the enemies of the opposite color.
