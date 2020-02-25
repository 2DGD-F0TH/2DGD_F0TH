\null\clearpage

Introduction to Game Design
============================

In this section we will talk about platforms, input systems and game genres, in a quick fashion. This chapter will introduce you to the language and terms used in game design, this way the following chapters will be easier to comprehend.

We will talk about the differences and challenges deriving from each decision and the basic way game genres work.

Platforms
----------

There are several different platforms a game can be developed for, and each one has its own advantages and drawbacks. Here we will discuss the most notable ones.

### Arcade

Arcade cabinets have been around for decades, and have still a huge part in the heart of gaming aficionados with classic series going on like "Metal Slug". The main objective of these machines is to make you have fun, while forcing you to put quarters in to continue your game.

These cabinets' software is known to be very challenging, having some nice graphics and sound. Arcade games are usually presented in the form of an "arcade board", which is the equivalent of a fully-fledged console, with its own processing chips and read-only memory.

In the case of arcades, the hardware is usually tailored to support the software; with some exceptions added later (like the Capcom Play System, also known as CPS), where the hardware is more stable between arcades, while the software changes.

### Console

Consoles are a huge (if not the biggest) part in the video game industry. Their Hardware is dedicated solely to gaming (and some very marginal "multimedia functionalities") and it evolves in "generations": this means that each "generation" has a stable hardware programmers can study and exploit.

This hardware stability is a double-edged sword: the hardware can be really hard to master at the beginning, resulting in some poor-performing games at the beginning of the generation, but when mastered the results are incredible. This feeds into a cycle that looks like the following:

1. New Generation is introduced
2. Initial confusion, with poor performance and graphics
3. Hardware is mastered and games have great performance/graphics
4. The games become "too big" for the current generation and a new generation must be introduced.

### Personal Computer

Personal Computers are another huge part of the video game industry. They are extremely flexible (being general-purpose machines) but have a huge drawback: their hardware is not the same from one unit to the other. This means that the programmer needs to use "abstraction layers" to be able to communicate with all the different hardware.

This can have performance costs, as well as forcing the programmer to add options to lower graphic settings, resolution and more.

All of this just to be able to run on as many computers as possible. The upside is that when the computer is really powerful, you can get great performance and amazing quality, but that's a rare occasion.

### Mobile

One of the most recent platforms game developers work on is right in your pocket: your smartphone.

Today's smartphones have enough power to run fully-fledged videogames, on the go. Sadly the touch screen can prove to be really uncomfortable to use, unless the game is specially tailored for it.

### Web

Another platform that has seen a massive rise in recent times is the Web: with WebGL and WebAssembly, fully-fledged games (including 3D games) can run on our browser, allowing for massively-multiplayer experiences (like Agar.io) without the hassle of manual installation or making sure the game is compatible with your platform.

A drawback of the "web approach" is the limited performance that web browsers, WebGL and WebAssembly can give, as well as the need to download the game before being able to play (and sometimes you may need to re-download the game if you cleared your browser's cache).

Input Devices
-------------

A game needs a way to be interacted with: this "way" is given by input devices. In this section we will take a brief look at the input devices available in a game.

### Mouse and Keyboard

One of the most common input devices, most of the currently available frameworks and engine have support for input via mouse and keyboard. These input methods are great for visual novels, point and click adventures, FPS/TPS games and anything that is considered to be "made for PCs".

### Gamepad

One of the classics of input devices, works well with the majority of games: FPS/TPS games may need some aim assist mechanic in your game. Point and click adventures feel clunky with this input method.

As with Mouse and Keyboard, most of the currently available engines and frameworks support gamepads.

### Touch Screen

With the coming of smartphones, touch screen is a new input device that we have to account for. Touch screens emulate computer mice well enough, although they lack precision.

The nature of being a mix between an input device and a screen brings a lot of new ways to experience a game if well done. Many times touch screens are used to simulate game pads: the lack of the tactile feedback given by buttons makes this simulation clunky and uncomfortable.

Some of the most recent framework and engines support touch screens, although there's an additional layer of complexity given by the specific operating system of the smartphone you're building for.

### Dedicated Hardware

Some games require dedicated hardware to work at their best, if at all. Guitars (guitar hero), wheels for racing games, joysticks for flying simulators, arcade sticks for arcade ports...

Dedicated hardware requires precise programming, and is usually an advanced topic. On PCs many "dedicated input devices" are recognized as "game pads" and use an "axis" and "buttons" abstraction that makes coding easier.

### Other Input Devices

A special mention is deserved for all the input devices that are "general purpose" (as in not "dedicated") but are still in a group outside what we saw so far.

In this group we see gyroscopes, accelerometers (like the Nintendo Wii/Switch JoyCons), sensors, IR as well as other exhotic hardware that can still be exploited in a videogame.

Game Genres
-----------

### Shooters

<!-- TODO: FPS, TPS, Top-down Shooters, anything with a projectile weapon, really -->

\placeholder

### Strategy

<!-- TODO: RTS, TBS, anything in the style of Civilization or Age of Empires -->

\placeholder

### Platformer

<!-- TODO: Any mario game, practically -->

\placeholder

### RPG

<!-- TODO: Role Playing Game, like LoZ or Final Fantasy, can be turn-based or action/adventure -->

\placeholder

### MMO

<!-- TODO: Any game with a massively multiplayer component -->

\placeholder

### Simulation

<!-- TODO: Farming, economy/business, social simulations like the sims, sim city or sim farm... -->

\placeholder

### Rhythm Games

<!--TODO: Based on how precisely you can follow the music beat, can be as simple as DDR or as complex as Crypt of the Necrodancer/Cadence of Hyrule -->

\placeholder

### Visual novels

<!--TODO: Choose-your-own-path graphical adventures that have "telling a story" as their main objective -->

\placeholder
