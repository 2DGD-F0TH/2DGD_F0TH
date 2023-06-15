{{pagebreak}}

A Game Design Dictionary
========================

:::::: {.epigraph author="Duane Alan Hahn"}
Why should you make games? Do it to give players joy from your unique perspective and to have fun expressing yourself. You win and the players win.
::::::

In this section we will talk about platforms, input systems and game genres, in a quick fashion. This chapter will introduce you to the language and terms used in game design, this way the following chapters will be easier to comprehend.

We will talk about the differences and challenges deriving from each decision and the basic way game genres work. The objective of this chapter is giving you some terminology and knowledge about game design, before deep-diving into the topic.

Platforms
----------

There are several different platforms a game can be developed for, and each one has its own advantages and drawbacks. Here we will discuss the most notable ones.

### Arcade

Arcade cabinets have been around for decades, and have still a huge part in the heart of gaming aficionados with classic series going on like "Metal Slug". The main objective of these machines is to make you have fun, while forcing you to put quarters in to continue your game.

These cabinets' software is known to be very challenging (sometimes due to the fact that you're popping quarters into the machine for the "right to play"), having some nice graphics and sound. Arcade games are usually presented in the form of an "arcade board", which is the equivalent of a fully-fledged console, with its own processing chips and read-only memory.

![How an arcade machine usually looks like](./images/game_design/arcade.svg){width=30%}

In the case of arcades, the hardware is usually tailored to support the software; with some exceptions added later (like the Capcom Play System, also known as CPS), where the hardware is more stable between arcades, while the software changes.

### Console

Consoles are a huge (if not the biggest) part in the video game industry. Their Hardware is dedicated solely to gaming (and some very marginal "multimedia functionalities") and it evolves in "generations": this means that each "generation" has a stable hardware programmers can study and exploit.

![A portable console](./images/game_design/console.svg){width=30%}

This hardware stability is a double-edged sword: the hardware can be really hard to master at the beginning, resulting in some poor-performing games at the beginning of the generation, but when mastered the results are incredible. This feeds into a cycle that looks like the following:

1. New Generation is introduced
2. Initial confusion, with poor performance and graphics
3. Hardware is mastered and games have great performance/graphics
4. The games become "too big" for the current generation and a new generation must be introduced.

### Personal Computer

Personal Computers are another huge part of the video game industry. They are extremely flexible (being general-purpose machines) but have a huge drawback: their hardware is not the same from one unit to the other. This means that the programmer needs to use "abstraction layers" to be able to communicate with all the different hardware.

![A personal computer](./images/game_design/pc.svg){width=40%}

This compounds with the fact that "abstraction layers" used by the developer (like SDL, SFML or GLFW) are running on top of other "abstraction layers", like sound servers, device drivers, etc... which can be littered with bugs themselves. Just look at how many indirections we have on a modern Linux system (which is usually bundled with PulseAudio):

![How many abstraction layers are used just for a game to be able to play sounds](./images/game_design/sound_abstraction.svg){width=30%}

This can have performance costs, as well as forcing the programmer to add options to lower graphic settings, resolution and more.

All of this just to be able to run on as many computers as possible. The upside is that when the computer is really powerful, you can get great performance and amazing quality, but that's a rare occasion.

### Mobile

One of the most recent platforms game developers work on is right in your pocket: your smartphone.

![A smartphone](./images/game_design/phone.svg){width=20%}

Today's smartphones have enough power to run fully-fledged video games, on the go. Sadly the touch screen can prove to be really uncomfortable to use, unless the game is specially tailored for it.

### Web

Another platform that has seen a massive rise in recent times is the Web: with WebGL and WebAssembly, fully-fledged games (including 3D games) can run on our browser, allowing for massively-multiplayer experiences (like Agar.io) without the hassle of manual installation or making sure the game is compatible with your platform.

![Fully fledged games can run in your browser nowadays](./images/game_design/browser.svg){width=40%}

A drawback of the "web approach" is the limited performance that web browsers, WebGL and WebAssembly can give, as well as the need to download the game before being able to play (and sometimes you may need to re-download the game if you cleared your browser's cache).

Input Devices
-------------

A game needs a way to be interacted with: this "way" is given by input devices. In this section we will take a brief look at the input devices available in a game.

### Mouse and Keyboard

:::: centering ::::
![](./images/game_design/keyboard.svg){width=40%}
:::::::::::::::::::

One of the most common input devices, most of the currently available frameworks and engine have support for input via mouse and keyboard. These input methods are great for visual novels, point and click adventures, FPS/TPS games and anything that is considered to be "made for PCs".

### Gamepad

:::: centering ::::
![](./images/game_design/gamepad.svg){width=40%}
:::::::::::::::::::

One of the classics of input devices, works well with the majority of games: FPS/TPS games may need some aim assist mechanic in your game. Point and click adventures feel clunky with this input method.

As with Mouse and Keyboard, most of the currently available engines and frameworks support gamepads.

### Touch Screen

:::: centering ::::
![](./images/game_design/phone.svg){width=20%}
:::::::::::::::::::

With the coming of smartphones, touch screen is a new input device that we have to account for. Touch screens emulate computer mice well enough, although they lack precision.

The nature of being a mix between an input device and a screen brings a lot of new ways to experience a game if well done. Many times touch screens are used to simulate game pads: the lack of the tactile feedback given by buttons makes this simulation clunky and uncomfortable.

Some of the most recent framework and engines support touch screens, although there's an additional layer of complexity given by the specific operating system of the smartphone you're building for.

### Dedicated Hardware

:::: centering ::::
![](./images/game_design/dedicated_hw.svg){width=40%}
:::::::::::::::::::

Some games require dedicated hardware to work at their best, if at all. Guitars (guitar hero), wheels for racing games, joysticks for flying simulators, arcade sticks for arcade ports...

Dedicated hardware requires precise programming, and is usually an advanced topic. On PCs many "dedicated input devices" are recognized as "game pads" and use an "axis" and "buttons" abstraction that makes coding easier.

### Other Input Devices

A special mention is deserved for all the input devices that are "general purpose" (as in not "dedicated") but are still in a group outside what we saw so far.

In this group we see gyroscopes, accelerometers (like the Nintendo Wii/Switch JoyCons), sensors, IR, as well as other exotic hardware that can still be exploited in a video game.

Game Genres
-----------

Let's analyze some game genres to understand them better and introduce some technical language that may be useful in [writing a Game Design Document](#GDD).

These genres are quite broad, so a video game is usually a mix of these "classes" (like a strategy+simulation game).

### Shooters

Shooters are games that involve... shooting. They can include any kind of projectile (bullets, magic from a fairy, arrows from a hunter) and can be crossed with any other genre (creating sub-genres in a way), like 2D platformers.

Some of the most known shooter genres are:

- **FPS** (first person shooters), 3D games where the game is shown from the point of view of the protagonist. This involves only seeing a HUD and the weapon, instead of the whole character;
- **TPS** (third person shooters), 3D games where the game is shown from a behind-the-character perspective. Some show the whole protagonist, while others adopt an over-the-shoulder perspective;
- **Top Down Shooters**, usually 2D games where you may be piloting a vehicle (space ship, plane, etc...) and shoot down waves of enemies, in this category we fit arena shooters (like Crimsonland) and space shooters (like Galaga);
- **Side scroller shooters**, usually 2D games and platformers, where you control the protagonist and shoot enemies on a 2D plane, in this category we find games like Metal Slug.

### Strategy

Strategy games involve long-term planning and resource control, they are slower games, but can be really intense when played in competition with other players.

Some of the most popular strategy genres are:

- **RTS** (real time strategy), where units are controlled in real time;
- **Turn-based strategy**, where units and resources are managed in turns;

### Platformer

Platformer games involve difficult jumps and precise movement, they can both be 2D and 3D games. A prime example of platformer games is the Mario series: Mario 1,2,3 for 2D games and Mario 64 for 3D.

### RPG

RPGs or "Role Playing Games" are games where you assume the role of a character in a fictional setting. In RPGs the world is well-defined and usually have some level or class system and quite advanced item management.

RPGs can be either action/adventure, with real-time actions, turn-based or hybrid, where the movement is done in real time but battles happen in turns. Some prime examples of RPG games are the Legend of Zelda series, as well as the Final Fantasy series.

### MMO

MMO (Massively Multiplayer Online) is a term used for games that have a heavy multiplayer component via the internet. The most known MMO genre is MMORPGs (Massively Multiplayer Online Role-Playing Games).

### Simulation

Simulation games cover a huge variety of games that are created to "simulate reality", in more or less precise ways. Among simulation games we can find:

- **Racing Games:** sometimes more simulative others more arcade-like, racing games simulate the experience of driving a vehicle, more or less realistic (from modern cars to futuristic nitro-fueled bikes);
- **Social Simulation:** simulating the interaction between characters, a pioneer on the genre is surely "The Sims";
- **Farming simulation:** simulating the quietude and work in the fields;
- **Business simulation:** like "game dev tycoon" or "rollercoaster tycoon";

But there are also other kinds of simulations, like Sim City, where you manage an entire city.

### Rhythm Games

Rhythm games are based on the concept of following a music beat as precisely as possible, this can be also used as a "mechanic" in other types of games.

Some examples of Rhythm games are "Dance-Dance Revolution" (also known as DDR), as well as more innovative games like "Crypt of the Necrodancer" (a mix between rhythm game and dungeon crawler).

### Visual novels

Visual novels are graphical adventures whose primary objective is "telling a story", they can be linear or have a "choose your own path" component. They usually feature multiple endings and hand-crafted still images as artwork.

The more modern versions feature more interactive components and fully-fledged 3D graphics, but what ties the genre together is usually a "point and click" style of gameplay.

Miscellaneous
-------------

Here we will talk about some other terms that you may hear in the game development and design world, but that don't fit into a specific category.

### Emergent Gameplay

Sometimes, when interacting with simple game mechanics, players can give life to complex situations. When that happens usually we talk about "emergent gameplay".

Emergent gameplay can take place in open-ended games, where there are many solutions to a situation and none of them is "preferred by the game". For instance, we can think of someone guarding a door, there are many ways to get through the guard, such as:

- Attacking the guard (and winning);
- Find an alternative path;
- Sneak around the guard to knock them unconscious;
- Find a way to make the guard leave their post;
- ...

::: trivia :::
A prime example of a game that leverages emergent gameplay is Minecraft. Players can either survive, build palaces, build redstone circuits and much more.
::::::::::::::
