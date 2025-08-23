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

Even though piracy infringes on someone's copyright and right to earn money from their hard work, "wagging their finger" and implementing strict anti-piracy measures might not be the best solution.

Piracy may hide some underlying issues: it is a "service problem"; we are missing something that may be considered fundamental by others.

### Regional Pricing

Not all regions have the same per-capita income. Let's make an example with data from the year 2025, both for income and price for triple-A games.

If we consider a triple-A game that costs about 80$ we can find out that:

- The US has a monthly per-capita income of about 5400$. A 80$ game would take up about 1.5% of such income. Some might consider it a fair price for entertainment, some may not (I think 80$ is too high, but I'm used to the good old times of 60$ games).
- Albania has a monthly per-capita income of about 850 to 1000$. An 80$ game would take up about 8% to 9.4% of such income. Starting to see a problem?
- In Brazil there is a monthly income that averages at about 612$. An 80$ game would take up a staggering 13% of such income. If a person earned minimum wage (barely scraping 275$) buying such a game would be crazy (taking up about 30% of their income).

So the lower-income countries may see a higher piracy rate for a simple reason: they cannot afford the game.

Having regional pricing that make your games possible to purchase in all regions can be a good way to cater to this audience: people who want to support you, but can't. Sometimes a simple currency-conversion is not enough, since it doesn't always keep track of the spending power of people.

### Give out free demos

Some people may just want to try a game, and it appears that giving out free demos is a practice that has been lost to time.

As usual, when a market vacuum is created someone will fill it. Pirates might be someone who fills such market vacuum by giving out full versions of games.

By giving out a scope-limited demo (the first few levels or the first chapter) you take away such market vacuum and give prospective players a chance to try your game before committing to a full purchase.

### No DRM entirely

{{placeholder}}

<!-- TODO: People who don't want to pay for a game, will find a way to crack it, while who does will most probably pay for it if they can afford it. -->

Implementing anti-piracy measures
---------------------------------

If you really want to implement anti-piracy in your game, you may want to take some precautions and make things harder for pirates, just to earn enough time to last for the majority of the sales period.

:::: note ::::
Note from the author: I don't like DRM systems in general. I find their modern DRM behaviour invasive and bordering on spyware-like. For fairness I will report on DRM and anti-piracy systems, but I don't really encourage their use, since it's really easy to border on "abuse".
::::::::::::::

### Third-party DRM systems

The best way to do something, is to make someone else do it.

As much as trying to make your own anti-piracy and Digital Rights Management system might sound exciting, it may soon become extremely tedious and not as effective as some solutions offered by players who have been working on the matter for years.

Steam has its own DRM which is, for better or worse, well-understood by pirates and easily defeated, but it's always better than "no DRM" when it comes to protecting your own copyright.

There are third party solutions that I won't list here, but if you really want to implement one, you can look for more information online: giving out names might date this book since solutions change quickly.

### Debugger detection

One way to stop the more "naive" trials to crack your game is debugger detection.

Implementations vary with your programming language, but here are various methods:

- On Linux you can check `/proc/self/status` and check for a value in the `TracePid` field, if it's not zero then a debugger is attached;
- On Windows, there is a handy `IsDebuggerPresent` function that returns a boolean;
- With JavaScript you can create a small script that takes the current date and time, calls the `debugger` keyword and then takes the date and time again. If the difference between the two dates is longer than an arbitrary length of time, a debugger might be attached. This works because the `debugger` keyword triggers the debugger only if a browser console is open.

Once you find that a debugger is attached, you can decide the best course of action, usually by closing the program or forcibly crashing it.

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
