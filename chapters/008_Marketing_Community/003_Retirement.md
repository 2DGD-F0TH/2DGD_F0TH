{{pagebreak}}

When the time for retirement comes
==================================

In all the computer science texts, you will find that the software life cycle is roughly divided in 6 steps:

1. Analysis;
2. Design;
3. Development;
4. Testing;
5. Deployment;
6. Maintenance;

But nothing is eternal, and thus sooner or later, we will need to talk about "software retirement".

Software retirement happens when a software reaches the so-called "end of support" stage (sometimes called "end-of-life"): the software doesn't get updated anymore, security patches are no applied and development stops. Sometimes even distribution stops, as well.

End of life can happen for different reasons:

- The platform your game is based on is not supported anymore, that includes: console, operating system, runtime libraries...
- Financial reasons: this usually involves multiplayer games, when the cost of server upkeep makes the maintenance of the game "antieconomic";
- Loss of interest from the community: the game may not be played as much, and thus maintenance may be a draining task (either financially or mentally);
- Simple lack of time: people have their own lives and sometimes they have to decide that something needs to be cut to make space for something else (newer projects or just personal time);

So what can we do when a game reaches its "end of support" phase? Let's see.

:::: note ::::
This section assumes that you're publishing the game yourself, or you have a deal with your publisher that allows what I'm about to show.
::::::::::::::

Remonetization
--------------

This is usually done by big companies when a console reaches its end of support phase: they update the game (or build it anew, if necessary) and release it on the new platform.

This has some advantages, such as:

- The game is preserved (at least for the foreseeable future);
- The game is updated for new platforms, thus extending its lifespan;
- The game can be "remastered", thus making it more appealing to newcomers (usually in the form of a graphical upgrade);
- The developer and publisher can re-sell the game, thus getting more funds for new projects.

There are also some disadvantages, as well:

- The developers (and publisher) will have to face an upfront cost for the upgrade (not knowing if the new sales will be worth the risk);
- The P.R. hit coming from "reselling the same thing to the same people";

Usually the second issue is addressed with substantial discounts for owners of the "older version" or sometimes even just and outright "free update" option.

Free release
------------

If the product is paid, the developers/publisher may decide to release the game for free if the situation allows it, sometimes even by removing DRM entirely.

This means that the game will continue to live past its "official life cycle" and new players may decide to try the game themselves.

When technical problems arise, there is a possibility that a community of fans will unite in the effort of building compatibility layers for the game and allow it to continue existing without the intervention of the original development team.

Sometimes those projects may even take the form of complete re-implementations of parts of the game, allowing for very old games to be experienced on modern systems. An example of this is "SCUMMVM" for adventure games: with it, you can play adventures as old as Maniac Mansion (1987) on modern PCs in 2024 (the time of writing this section, that's 37 years!).

Open Sourcing
-------------

A more radical version of releasing a game for free, is taking the source code and publish it on a versioning platform with a permissive open-source license.

This allows the community to take over the maintenance of the game they love, without the effort of having to "reverse engineer" parts of the engine to make things work.

This is essentially giving the game away for free and allow people to look and edit the source code: this way any person who wants the game to be preserved can tweak it until it works natively on the device they want it to work on, or just analyze the source code and build the necessary "compatibility layers" to make it work.

In case of multiplayer games, it is ideal to release the server code too, allowing players to build their own servers and continue enjoying what you created for years to come (eventually by doing a final code commit on your game, allowing for custom server IPs to be used).

:::: note ::::
Author's note: this is the way I prefer things to go, personally. I think it's just ideal.
::::::::::::::

### When "Open Sourcing" is not enough

I want to mention a game where sadly open-sourcing didn't do much to save it: the name is "Mother ZerOS: An open source hacking simulation" by Massimo "V4ldemar" Pinzaglia (which itself is the spiritual successor to "Mother: A computer hacking simulation" by the same author).

It was supposed to be an open source "Hacking simulation" game, much in the style of "Uplink". The game and source code (made using IRRlicht) were downloadable from V4ldemar's website: sadly (from the news I could find) V4ldemar left us in late 2018, his domain has expired and the game has disappeared from the face of the internet.

That is because the source code (as well as the installers) was available exclusively from his website and the website itself used a mix of PHP and AJAX requests (probably to save on bandwidth), thus stopping scrapers (Like Internet Archive's "Wayback machine") from processing the website correctly.

The game was in alpha state, but it had a lot of charm and it is painful to know that such a piece of my childhood is lost forever.

Hybrid Approaches
-----------------

A possible hybrid approach could be releasing the engine as open source but keeping the art and assets under a "All copyrights reserved" license instead.

This is a way to protect your intellectual property (in case you want to remonetize it down the road), while allowing the current user base to at least make things work and do their own ports, but the graphics, setting, sound, levels and characters will remain under your control.

This is the solution that was adopted by DOOM developer "id Software", when they released the DOOM engine source code under a "not-for-profit license" (later changed to GPL2) but kept the assets proprietary.

A hybrid approach is the ideal way to allow the community to have a game supported by other (more tech-savvy) members of the community, while avoiding "diluting" the value of the brand you created.

What not to do
--------------

And here we are, the event that sparked this whole section of the book and what not to do when dealing with the "end of support" phase of your game.

On December 14th 2023, Ubisoft declared that their online racing game "The Crew" was going to be retired from market "due to upcoming server infrastructures and licensing constraints".

The game was retired from storefronts and could not be sold anymore.

Sadly this is not the end of it, because the game stopped being playable in April 1st 2024: the servers were taken down and the game (being "always online" even in its "single player form") stopped working.

But that is not all, on or around April 14th 2024, people started noticing that the game's license has been revoked. This is the equivalent of a publisher entering your home and taking away your game's disc.

This is a textbook example of what **not to do** when your game reaches its end of support phase.

::: note :::
What follows **is not legal advice**. I am not a lawyer.

If you want to know more (as in quantity and quality of information), contact your favourite lawyer. What follows is just my somewhat objective evaluation of the situation, as well as an opinion.
:::::::::::::::

First of all, the "server infrastructure limitations" issue could have been solved by allowing players to host their own servers. Releasing the source code for the server was not necessary, since an executable form of the server would have been enough for players to be able to enjoy the game they have spent their hard-earned cash on.

The only way I think these limitations could come to be, without being able to give the community a self-hostable server, would be that the current server infrastructure is shared between games, which is somewhat worrisome.

The licensing issue, I assume is due to in-game music, which I don't think should be a problem, because the publisher decided to stop selling the game. That means that they should not infringe on any license, since the game is not sold anymore.

Again, the only way I think there could be licensing issues, is that the servers are themselves using pieces of software that are subject to a license agreement. That would be a reason to not give out a self-hostable server, because doing so would infringe on the software license of some components of the server itself (which may have expired).

But even accounting for all these motivations, there is absolutely **no excuse** (ethically speaking, I'm sure the EULA has something that allows them to do that) for Ubisoft to just decide to revoke the licenses for a product that people paid for.

There have been some amazing projects from the fans who managed to bring games "back to life" from seemingly desperate situations, just because the fans were passionate enough analyze every last bit in the effort to reverse engineer a way to play a beloved game again.

:::: trivia ::::
An example of a game that was "brought back from the dead" is "Sonic Runners", a game that was released in June 2015 and retired July 2017, when the servers were shut down, making the game unusable. Through reverse engineering, a team of fans managed to reverse engineer a client and server, making it playable again.
::::::::::::::::
