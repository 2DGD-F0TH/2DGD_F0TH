{{pagebreak}}

Dealing with piracy
===================

:::::: {.epigraph author="Gabe Newell"}
One thing that we have learned is that piracy is not a pricing issue. It’s a service issue. The easiest way to stop piracy is not by putting anti piracy technology to work. It’s by giving those people a service that’s better than what they’re receiving from the pirates.
::::::

When you're selling a videogame (or anything that is covered by copyright), you will sooner or later have to deal with piracy.

Piracy usually refers to the act of distributing (either physically or digitally) copyrighted works (including music, software and movies) without the copyright holder's permission, usually regardless of whether such distribution is done for free or not.

:::: note ::::
There are some exceptions to forbidding the physical distribution of copyrighted goods: you usually can sell your regularly purchased physical books, movies and games, but you cannot make a copy of them and distribute that. For more information about the matter search for the terms "first-sale doctrine".
::::::::::::::

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

Let's face it, if someone doesn't want to pay for a game, they will find a way to not pay for it, either by finding a cracked copy, or by cracking it themselves (provided they have tools and knowledge on how to do so). It's a sad reality.

Considering how invasive DRM systems have become (including but not limited to Kernel-mode drivers which sometimes almost behave like spyware) and their consequences on performance, people are becoming more and more conscious.

So the choice of using no DRM at all might be considered favourably by the general public, to the point it might be considered a positive feature.

### Poisoning the well

I'm putting this as an "alternative" because you don't really need a DRM to enact this.

An alternative could be putting a "pirate version" out there yourself: many groups see pirating software as challenges to overcome, thus putting out a fake "pirate version" of the game may stop the pirates from targeting your game for a bit.

The secret is that such "special version" is actually different from the retail version: it is built in such a way that it either crashes randomly, has a game-breaking bug down the story or misbehaves in such a way that makes it impossible for the person who downloaded the pirated copy to enjoy the game as it should be. Your fantasy is your friend here, just make things believable enough that the bug you introduce can be chalked to a "bad release" or something else but a honeypot.

It's a dirty tactic, but it can prove to be effective.

:::: trivia ::::
Game dev Tycoon has implemented a similar tactic, but also had some "call home" code that notified the developers about how many copies of their game were pirated. As much as it might seem innocent from an outside view (it's just statistics), this wasn't well-received by all gamers (including the ones who purchased the game) because of its spyware-like behaviour.
::::::::::::::::

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

When implementing anti-piracy checks, you should not show any piracy-related message: those messages can be traced in the executable file (the `strings` GNU/Linux command can show all the "printable strings" inside an executable) and thus give a lead on where your anti-piracy code is located, making its removal that much easier.

When an anti-piracy is triggered, you should try and hide it behind a generic error that is already used somewhere else (the more often such error is used, the better, like hiding a tree in a forest). Such error can crash the program and deter some people by making them think that the pirated copy is low quality.

Another way could be tinkering with the game variables and make the game misbehave worse and worse until it's unplayable or it crashes.

#### Play with the pirates

Another way could be toying with the people who pirate the game: to make it more or less obvious that you know they're pirating the game.

Here are some examples:

- Game dev tycoon: after playing a while, your studio will go bankrupt due to piracy;
- The Talos Principle: you get stuck in an elevator;
- Crysis Warhead: guns shoot chickens which deal no damage;
- Alan Wake: the character wears a pirate-like eyepatch for the whole game;
- Serious Sam 3: an invincible, super-fast and enraged Arachnid spawns, following you for the whole time you are alive.

Your game does not need to become unplayable, that will only annoy the players and make them hate you, but a stern warning essentially saying "hey, we know you didn't buy this game" might have a bigger effect than you think.

Some situations may even lend them to occasions where the players are led to publicly oust themselves as people who are not playing a legitimate copy: if a problem pops out that seems a bug, but is actually something that happens only to pirated games, the player may open a support ticket asking for information which will just get a "if you stop pirating the game, it will fix itself" as a response.

### Don't stop the pirates immediately

When a piracy flag is triggered, you shouldn't immediately crash the game. Crashing the game immediately will tell whoever is trying to pirate the game that their edits to the executable have been detected, giving them an immediate feedback on their job.

If you slowly begin introducing issues, or wait for a few minutes or hours before crashing your game, you will slow down the pirates.

Even better if, when a piracy check is triggered and a game crashed, that every save file is wiped, forcing the pirate to re-play a good portion of the game at least once (before they start backing up their save files) to see if their edits work.

Another interesting thing to implement would be wiping the saves at the game boot if a flag has been set (for instance in the settings, with a name completely unrelated to piracy, again "hiding a tree in the forest"), making restoring save files harder or even meaningless.
