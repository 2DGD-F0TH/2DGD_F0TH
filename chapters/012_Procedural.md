{{pagebreak}}

Procedural Content Generation
=============================

:::::: {.epigraph author="Donald Knuth"}
Science is what we understand well enough to explain to a computer. Art is everything else we do.
::::::

What is procedural generation (and what it isn't)
-------------------------------------------------

Sometimes you hear "procedural generation" being thrown around as a term describing that some part of a videogame is generated with an element of randomness to it.

This isn't entirely true, since "procedural generation" suggests the presence of a "procedure to generate the item", in short: an algorithm.

A procedurally generated weapon is not statically created by an artist, but instead by an algorithm that puts together its characteristics. If the algorithm has the same data in its input, then the same item will be generated as an output.

When you introduce an element of randomness (or more precisely **pseudo-randomness**) you have what is called "random generation".

Let's make a simple example: we want our Super-duper-shooter to make use of procedural/random generation to create your weapons. The following example will clarify the difference in algorithms between procedural and random generation, all weapons have a body, a scope, a barrel and an ammo magazine.

This is a possible algorithm for a procedural weapon:

```{src='procedural/createproceduralweapon' caption='Example procedural weapon creation'}
```

This instead is a possible algorithm for a random weapon, for simplicity we assume that the pieces are all compatible:

```{src='procedural/createrandomizedweapon' caption='Example Randomized weapon creation'}
```

As you can see, the algorithms are very similar to each other, but the second one has an element of randomness added to it.

So, as a memorandum:

Procedural generation is **consistent**, even though something is generated in real time, given the same input the same output will be returned.

Random generation is usually **not consistent**, although it is possible to control the random generator (via its seed) to obtain deterministic results, given the same input.

Seeding a random number generator correctly can allow you to generate a huge universe without storing it into memory, for instance; although the edits to such universe will have to be saved in some other way.

Advantages and disadvantages
------------------------------

As with everything, procedural and random generation has its advantages and disadvantages, which will be explained below.

### Advantages

#### Less disk space needed

Using algorithms to build worlds and items means generating them mostly in real-time, which means we don't have to save them to hard-disk, since if the algorithm is not randomized, you can always re-create the same worlds and items when requested. This was more pressing at the times of the NES, where game sizes were usually around a couple hundreds of KB.

#### Larger games can be created with less effort

When a world is handcrafted, everything has to be placed and textured manually, which takes time and money. This obviously puts a superior limit to how big these worlds can be.

When procedural (and randomized) generation comes into play, there is no theoretical limit to how big these worlds can be (considering an infinitely powerful hardware).

Same goes for items, each handcrafted item takes time and money, while using procedural generation you can re-use components of said items to generate a potentially infinite number of new items that have certain characteristics.

#### Lower budgets needed

Creating a videogame is expensive, in fact the so-called "AAA" games costs are in the order of millions of dollars. Using procedural and random generation you can create variations of your resources (textures, for instance), lowering costs.

#### More variety and replayability

When a world and its objects are handmade, the game experience is bound to be fixed: same items to collect, same world, same overall experience. Procedural and random generation can bring some sense of "unknown" to the game every time you play. This also enhances the replayability value of the game.

### Disadvantages

#### Requires more powerful hardware

Procedural generation makes use of algorithms, and such algorithms can be really taxing on the computer hardware, so loading times might increase or users with less powerful computers might experience stutters as their computer cannot "keep up" with the game demands.

#### Less Quality Control

Computers are able to crunch numbers at an incredible rate, but they lack creativity. In a procedurally generated world you lose the "human touch" that can introduce subtleties and changes that can be brought by a good designer with experience.

At the same time, there is a variation in user experience, so you cannot guarantee the same gameplay quality to all players. Some players may find a really easy map to play in, while others might find a really hard map that prohibits such gameplay.

#### Worlds can feel repetitive or "lacking artistic direction"

Consequence of having less quality control, worlds and items might feel like they "lack artistry", as well as being repetitive.

If you use procedural and randomized generation, you have the chance of generating incredibly large worlds with a huge variety of items with less resources and algorithms; that's where our human nature of "recognizing patterns" crashes the party: repeating patters are really easy to spot and can remove us from the game's atmosphere and introduce us to one of our worst enemies: boredom.

This can become even worse if we try to "find the middle ground" and build our levels using hand-made "chunks", joined together. If we want to avoid our player getting bored by repeating "pieces of level" we will need to build a lot of chunks that fit together to make something interesting and new almost every time.

#### You may generate something unusable

In extreme cases, there is a possibility that we end up generating an unplayable world, or useless items: terrain too high to climb, walls blocking a critically-necessary area, dungeon rooms with no exits, etc...

#### Story and set game events are harder to script

Being uncertain, procedural generation makes set events harder to script, if not impossible. In this case it's more common to use a mix of procedural generation and pre-made game elements, where the fixed elements are used to drive the narrative and the procedurally generated elements are used to create an open world for the player to explore and vary its gameplay experience.

Where it can be used
--------------------

Procedural (and random) generation can be used practically anywhere inside of a videogame, some examples could be the following:

- **World Generation:** Using an algorithm called "Perlin noise", you can generate a so-called "noise map" that can be used to generate 3D terrain, using giving areas with higher concentration a higher height. For dungeon-crawling games you might want to use a variation of maze generation algorithms, and so on so forth;
- **Environment Population:** You can use an algorithm to position certain items in the world, and if an element of randomness is required, positioning items in a world is certainly a very easy task and can add a lot to your game, but be careful not to spawn items into walls!;
- **Item Creation:** As stated previously, you can use procedural generation to create unique and randomized items, with different "parts" or different "stats", the possibilities are endless!;
- **Enemies and NPCs:** Even enemies and NPCs can be affected by procedural (and randomized) generation, giving every NPC a slightly different appearance, or scaling an enemy size to create a "behemoth" version of a slime, maybe by pumping its health points too, randomizing texture colors, again the possibilities are endless;
- **Textures:** It's possible to colorize textures, giving environments different flavours, as well adding a layer of randomness to a procedurally generated texture can greatly enhance a game's possibilities;
- **Animations:** An example of procedurally generated animations are the so-called "ragdoll physics", where you calculate the forces impacting a certain body (and it's "virtual skeleton"). A simpler way could be making the program choose randomly between a set of pre-defined "jumping animations" to spice up the game;
- **Sounds:** You can use sound manipulation libraries to change the pitch of a sound, to give a bit of randomness to it, as well as using "sound spatialization" by changing the volume of a sound to make it come from a certain place, compared to the player's position;
- **Story:** In case you want to put some missions behind a level-gate, you can use procedurally generated missions to allow the players to grind for experience and resources, so they are ready for the upcoming story checkpoints;
- **Difficulty Management:** Procedural generation can be involved into difficulty management by handing the enemy parameters and spawning to suit our needs.

Procedural Generation and Difficulty Management
-----------------------------------------------

As stated above, we can use elements of procedural generation to aid us in managing the difficulty of our game, tweaking the challenge we are offering to our players.

### Static difficulty

Sometimes called "Algorithmic Difficulty", is the kind of difficulty management seen in many games: elements and enemies are placed in a way that gives the player an ascending difficulty curve, with different parameters.

Such parameters build the abstract concept of "difficulty level" (beginner, normal, advanced, master, ...): each difficulty level contains a set of parameters that change how the game feels.

In practice usually you assign a "difficulty level" to an area and make the game handle the enemy spawn accordingly; sometimes such "area level" is used to make the game spawn randomized items with a certain power level, given a certain amount of displacement of its statistics.

### Adaptive Difficulty

Not really considered "procedural generation", adaptive difficulty makes use of different algorithms (taken on a lease from AI programming) to tweak the game difficulty in reaction to how the player plays.

If the player progresses quickly, the game will become harder, instead if the player tends to lose lives, the game will become easier and less random.

The objective of adaptive difficulty is to create an "optimal experience" for everyone, by determining if the current game is "too easy" or "too hard" for the player.

#### Rubberbanding

{{placeholder}}

<!-- TODO: Usually done in racing games, it's akin to the other runners/players being "tied to you with a rubberband", if you go too far ahead or behind, they will be "pulled towards you", by either speeding up or slowing down. This is also done by other games like NBA Jam by "tweaking" player skills to keep the game challenging -->

### Static vs. Adaptive Difficulty

Each approach has its own advantages and shortcomings, which can make one or the other better suited for your game.

Static difficulty is easy to create and leaves choice to the players between varying levels of difficulty; maybe someone wants a more "relaxing experience" instead of being continuously challenged.

The biggest shortcoming is that each level of difficulty is an estimate of its difficulty, so an "easy mode" may be way too easy, while the "normal" mode may be too hard for someone. It's hard to find the balance.

Also there's all the work dedicated to program the parameters for each level of difficulty.

Adaptive difficulty is harder to code and sometimes can lead to great results, but it also completely invalidates the concept of "grinding" in an RPG game, for instance. If you try to become stronger by undertaking easier quests, you will find that the quests keep getting harder the stronger you get.

Adaptive difficulty doesn't allow the player to "grind their way out" of a difficult part of the game.
