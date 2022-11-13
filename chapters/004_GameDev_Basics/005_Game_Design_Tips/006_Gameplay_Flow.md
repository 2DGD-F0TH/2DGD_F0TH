Designing the story and gameplay flow
-------------------------------------

When we are preparing the terrain for our game, it is vital to have an idea of how the story and the gameplay will unfold. There are lots of different types of gameplay, here we present some of them.

### Linear Gameplay

This is the simplest type of gameplay design: all story events come one after the other, without any possibility of deviating from the flow.

![Example Scheme of linear gameplay](./images/game_design/linear_gameplay.svg){width=60%}

Very much like a presentation, there is no branching, but such linearity can present some advantages, like ease of testing and possibility of applying traditional storytelling tools which have been developed for thousands of years.

Table: Summary of linear gameplay

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Linear Gameplay

**Advantages**            Simple and cheap to test, traditional storytelling tools can be used easily.

**Disadvantages**         No replayability, the gameplay may not feel very "interactive"
------------------------------------------------------------------------------


### Branching gameplay

Going towards more complex flow types, we can use branching to allow for more interactivity.

![Example Scheme of branching gameplay](./images/game_design/branching_gameplay.svg){width=50%}

This type of gameplay flow allows for a lot of interactivity by crafting the game in a way that player decisions have a direct influence on the story flow.

This gameplay flow is harder (and thus more costly) to test, but allows for multiple endings.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Branching Gameplay

**Advantages**            Simple to implement, allows for a strong feel of interactivity, allows for a lot of replayability by giving gameplay paths.

**Disadvantages**         Hard and costly to test, can get out of hand if not managed correctly.
------------------------------------------------------------------------------

### Parallel gameplay

The Branching gameplay flow has a huge disadvantage: it can be really hard to manage and doesn't really suit well "more linear" games.

Here's where parallel gameplay comes into play.

![Example Scheme of parallel gameplay](./images/game_design/parallel_gameplay.svg){width=60%}

In this flow style, there are branches running "parallel" to one another, but merge into "mandatory events" (which are usually story related). This way we have varied gameplay while keeping the story essentially linear.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Parallel Gameplay

**Advantages**            Moderately expensive to test, some traditional storytelling tools can be used, story is easier to manage.

**Disadvantages**         Replayability suffers from a story standpoint. If not well-made the player will feel like the story is "on rails" from the get go.
------------------------------------------------------------------------------

### Threaded gameplay

A different kind of gameplay is the "threaded" version, where there are many "beginnings", "middles" and "endings", usually done by playing different characters.

![Example Scheme of threaded gameplay](./images/game_design/threaded_gameplay.svg){width=60%}

This gives more replayability by giving many different and intertwining stories that allow to better understand a "bigger picture" of some sort. This gameplay flow can be costly, since it requires testing all the possible paths and crossings.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Threaded Gameplay

**Advantages**            Good replayability, great for giving many "sides" to a story.

**Disadvantages**         Testing all the paths can be costly, more difficult to manage.
------------------------------------------------------------------------------

:::: trivia ::::
This was done in Resident Evil 6, where different characters (and teams) have different stories that overlap.
::::::::::::::::


### Episodic gameplay

A more "object-oriented" approach to storytelling can be done by making small "episodes" (like mini-stories) with many entry and exit points.

![Example Scheme of episodic gameplay](./images/game_design/episodic_gameplay.svg){width=60%}

We need to be mindful of loops (we don't want to replay an episode that was already completed) when laying out our episodes. This gameplay flow allows for great interactivity, but kind of "forces" replaying the game to see all the possible episodes.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Episodic Gameplay

**Advantages**            Great interactivity.

**Disadvantages**         Tends to "force" replaying the game to see all episodes and paths, hard (and thus costly) to test and manage.
------------------------------------------------------------------------------

### Adding parallel paths

Nothing forbids us to mix and match methods to create something that suits our game better.

A very much appreciated and used gameplay flow is having a linear story with lots of "side quests" to give some diversion from normal gameplay, as well as replay value, since people are bound to miss some side quests.

### Looping Gameplay

This is a typical gameplay flow of roguelike games, where the player has to play the same game many times, beginning to end, eventually advancing a "bigger story".

![Example Scheme of looping gameplay with a overarching story](./images/game_design/looping_gameplay.svg){width=60%}

The most important thing when laying out a looping kind of gameplay is that the world needs to change between each "run": either by adding new weapons/items/collectibles or by unlocking a new part of the story (new levels, for instance) or adding new characters. Each run should feel like unique by itself.

----------------------    ----------------------------------------------------
**Gameplay Flow Type**    Looping Gameplay

**Advantages**            Great replayability.

**Disadvantages**         Needs a lot of care in laying out how the runs evolve between one and the next: if all the runs "feel the same" the player will abandon the game.
------------------------------------------------------------------------------
