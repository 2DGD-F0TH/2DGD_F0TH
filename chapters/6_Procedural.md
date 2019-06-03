\null\clearpage
Procedural Content Generation
=============================

What is procedural generation (and what it isn't)
-------------------------------------------------

Sometimes you hear "procedural generation" being thrown around as a term describing that some part of a videogame is generated with an element of randomness to it.

This isn't entirely true, since "procedural generation" suggests the presence of a "procedure to generate the item", in short: an algorithm.

A procedurally generated weapon is not statically created by an artist, but instead by an algorithm that puts together its characteristics. If the algorithm has the same data in its input, then the same item will be generated as an output.

When you introduce an element of randomness (or more precisely **pseudo-randomness**) you have what is called "random generation".

Let's make a simple example: we want our Super-duper-shooter to make use of procedural/random generation to create your weapons. The following example will clarify the difference in algorithms between procedural and random generation, all weapons have a body, a scope, a barrel and an ammo magazine.

This is a possible algorithm for a procedural weapon:

~~~~
function createProceduralWeapon():
    load body "body0001.png"
    load scope "scope0051.png"
    load barrel "barrel0045.png"
    load ammo magazine "mag0009.png"
    put together the loaded pieces
    set weapon damage to 45
    set weapon range to 15
    set weapon spread to 23
    return generated weapon
~~~~

This instead is a possible algorithm for a random weapon, for simplicity we assume that the pieces are all compatible:

~~~~
function createRandomizedWeapon():
    load a random body from the folder "weaponBodies/shotguns"
    load a random scope from the folder "weaponScopes/shotguns"
    load a random barrel from the folder "weaponBarrels/shotguns"
    load a random ammo magazine from the folder "weaponMagazines/shotguns"
    put together the loaded pieces
    set weapon damage to a value between 35 and 50
    set weapon range to a value between 13 and 18
    set weapon spread to a value between 20 and 30
    return generated weapon
~~~~

As you can see, the algorithms are very similar to each other, but the second one has an element of randomness added to it.

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
- **Sounds:** You can use sound manipulation libraries to change the pitch of a sound, to give a bit of randomness to it, as well as using "sound spatialization" by changing the volume of a sound to make it come from a certain place, compared to the player's position.
