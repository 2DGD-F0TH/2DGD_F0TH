Loading Screens
---------------

Loading screens can be subject to design too and deciding what to put in them can really enhance the player experience with your product.

### What to put in a loading screen

"What can we put in a loading screen?" The answer may sound obvious to some, but a simple loading screen has a lot going on. Let's think about it, the most barren loading screen imaginable has at least two elements:

- An animation, to make the loading screen less boring and to ensure the player that our game didn't lock up;
- Some kind of progress indicator, to let the player know how far the loading routine has gone.

But we can put lots more into it, let's take a

- **Story:** In story-heavy games, it may be a good idea to put a reminder, a briefing or just a couple sentences telling what's happening story-wise. This was done in DOOM (2016).
- **World Building:** If the story is not-so-linear, a good idea could be just telling some facts about the world of the game, what some primary NPCs like, their habits, etc...
- **Tips:** Putting some tips to help the player is one of the most common things done with loading screens, these tips should be short and useful (no "press F to kill your enemy silently", that's basic controls).
- **Minigames:** If your game may take a potentially long time to load, making the player play a simple mini-game (maybe with rewards) while they wait. This was done on the Playstation 2 (and can be enabled on the PC version) of Okami, where you can earn demon fangs with two minigames.
- **Status-related sentences: ** This is something that can serve both as a "loading screen filler" as well as "hidden debug feature"
    - **Actual loading information:** Some players may like knowing what their PC is doing, so showing "loading backgrounds" or "loading sound data" is a nice screen filler and can give your players a pointer in case the game locks up while loading.
    - **Funny phrases: ** Instead of boring, actual loading information, you can put funny phrases like "inserting buckazoids" (Space Quest, anyone?), if you have a list of funny phrases connected to actual loading checkpoints, it will function as a small debug helper.

### Letting the player "exit" the loading screen

In some cases it can be a good thing to have a loading screen fade into the next stage (or area) directly, while in other situations it may be wiser to prompt the player to "exit" the loading screen themselves (maybe by a "Press any key to continue" prompt).

The most important factor is whether the areas we will load into are safe: if the player is not ready, they may get killed by enemies, and that feels unfair.

You should use a "press to continue prompt" when at least one of the following conditions applies:

- **Any area we load to may be unsafe:** We don't want our players to take a small break, walk around the room and come back to a dead character;
- **There is text on the loading screen:** Be it a tip, world building or story, the player may be reading it, and taking away the text will annoy them.

### Avoiding a loading screen altogether

Dynamic loading is a technique that is usually implemented in 3D games, but nothing stops you from using it in your 2D game!

The main issue is avoiding the player noticing that you unloaded a piece of the map that you previously visited, while loading the next piece. This means:

- **Avoiding "popping":** the player won't appreciate seeing pieces of the game appear or disappear in front of their eyes;
- **Avoiding slowdowns:** the player will immediately realize something is going on if the game slows down or drops frames at a certain point. Plus it will feel like the game is not well-optimized.

Popping can be avoided in many ways, the most common one can be summarized with "what's outside the screen doesn't exist": if more than a single room fits in your screen, what's outside the screen space is a good candidate for garbage collection.

An interesting idea could be making use of the player's point of view to try and foresee which room the player will head to next. With a clever use of doors and "cone of vision" you can unload rooms that are inside the screen space.

This can be done by "cutting" the player's field of vision using doors and unloading a kind-of far-away room that may be seen if the door was open.

Another way could be using so-called "points of no return": rooms where you can't go back, forcing the player to continue on a certain almost-linear path. Be careful to not use too many of them, though! Players don't really appreciate seeing possibilities cut off from them.

:::: tip ::::
Elevators make good dynamic loading rooms: you're changing floors, so it makes sense to not being able to see anything outside and having an elevator animating while changing floors can "hide" that a loading operation is happening.
:::::::::::::
