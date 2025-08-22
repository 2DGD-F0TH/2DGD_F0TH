{{pagebreak}}

Dealing with piracy
===================

:::::: {.epigraph author="Gabe Newell"}
One thing that we have learned is that piracy is not a pricing issue. It’s a service issue. The easiest way to stop piracy is not by putting anti piracy technology to work. It’s by giving those people a service that’s better than what they’re receiving from the pirates.
::::::

{{placeholder}}

<!-- TODO: Gentle introduction on what piracy is -->

Objectives of anti-piracy measures
----------------------------------

Many people think that the main objective of anti-piracy measures is to stop people from copying the game and distribute it for free, thus damaging the companies that develop and publish the game.

That is true in part, the real objective is to stop people from distributing the game in the first few weeks of a game's life cycle: that's where most of its earnings lie.

A game usually makes from 50 to 70% of its earnings in the first month, if we don't consider DLCs and expansions, so if an anti-piracy system is able to ward off pirates for that period, the majority of earnings can be saved.

After that period, the damage from cracked anti-piracy is diminished, since the interest towards a game will fade with time. After a while, you may even think about just removing anti-piracy to improve performance (its impact is always non-zero) or to boost preservation efforts.

Alternatives to anti-piracy measures
------------------------------------

{{placeholder}}

<!-- TODO: Discuss how anti-piracy is not the best solution, as it usually hides other issues that can be solved without an anti-piracy system -->

### Regional Pricing

{{placeholder}}

<!-- TODO: Not all regions have the same per-capita income, so a straight currency conversion may mean that a game is essentially inaccessible to some regions due to the price being too high -->

### Give out free demos

{{placeholder}}

<!-- TODO: Some people who pirate games might just want to try the game, giving out a free demo removes that "market vacuum" from the hands of pirates. -->

### No DRM entirely

{{placeholder}}

<!-- TODO: People who don't want to pay for a game, will find a way to crack it, while who does will most probably pay for it if they can afford it. -->

Implementing anti-piracy measures
---------------------------------

{{placeholder}}

<!-- TODO: If push comes to shove, describe some methods to slow down pirates -->

### Third-party DRM systems

{{placeholder}}

<!-- TODO: Talk about some third-party DRM systems, like Steam's -->

### Debugger detection

{{placeholder}}

<!-- TODO: Windows has some APIs that allow for detecting if a debugger is attached, for instance -->

### Obfuscation and redundancy of checks

{{placeholder}}

<!-- TODO: Obfuscating strings and making multiple copies of the anti-piracy checks is a good way to slow down the cracking process -->

### Checksums

{{placeholder}}

<!-- TODO: Talk about checksums as a way to detect executable tampering -->

### When an anti-piracy flag is triggered, be generic

{{placeholder}}

<!-- TODO: Showing a warning that you know the game is pirated will just make it easier to detect where the anti-piracy check is. It's better to just crash the program or raise a generic error that already exists in plentiful quantities throughout the code base. -->

#### Play with the pirates

{{placeholder}}

<!-- TODO: Sometimes a good way to make life difficult for the pirates is playing with them. For example Serious Sam 3 Invincible Arachnid. -->

### Don't stop the pirates immediately

{{placeholder}}

<!-- TODO: When a piracy check flag is triggered, the game shouldn't stop immediately at boot: that will make it easier to detect where the  check is and tighten the feedback loop. It's better to let the people play a chunk of the game, slowing down the testing phase or even making them throw out a bad pirated copy. -->
