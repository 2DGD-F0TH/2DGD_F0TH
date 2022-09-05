{{pagebreak}}

Engines, Libraries And Frameworks
=================================

Here follows a list of game engines, libraries and frameworks, so you can make an informed choice about your platform, or just try one!

For each proposed engine or framework, along with a short description, you will find the following sections:

- **Website:** This contains a link to the official website of the engine/framework/library;
- **Price:** Here you will see if the product is free to use or if you need to pay a price for it, and anything in between;
- **Relevant Bindings:** In this table you will find if the product supports one of the many famous programming languages available;
- **Other Bindings:** Differently from the previous table, this table has only two columns:
    - **Proprietary Language:** This engine/framework makes use of its own scripting language, usually easier to learn than general-purpose languages. You may need to learn it;
    - **Visual Programming:** This product makes use of a "Visual Scripting" (codeless) paradygm, where you can program your own game without writing code, usually by using directed graphs and nodes.
- **Platform compatibility:** This will tell you if the product is good for your objective, by telling you which platforms you can deploy on, like Linux, Windows or even on the web via HTML5;
- **License:** This will tell you how to behave when it comes to the legal stuff, since some engines do not allow commercial use in their free versions.

General Purpose
---------------

General purpose engines are the basic frameworks that allow you to build any game you want, without being tied to a specific genre. These are the tools that give you the most freedom, but also will be a bit harder to master.

### ENIGMA {.unnumbered .unlisted}

**Website:** <https://enigma-dev.org/docs/Wiki/ENIGMA>

**Price:** Open Source

ENIGMA (Extensible Non-Interpreted Game Maker Augmentation) is an environment that is derived from Game Maker, but with time has taken a different approach than its counterpart: it compiles its own language (EDL) to C++ and it also allows to tie into C++'s templates and functions.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓          |            |            |            |            |            |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
| ✓                    | ✓                  |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

ENIGMA's license is GPL3, with a special exhemption that allows you to sell the products made with the software.

### Game Maker Studio {.unnumbered .unlisted}

**Website:** <https://www.yoyogames.com/gamemaker>

**Price:** Commercial

Game Maker Studio is one of the simplest game-making toolkits available on the market, but that doesn't mean it's not powerful. In fact, one of the most famous games of recent history was made with it: Undertale.

It makes use of its own scripting language, and some visual toolkits as well.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            |            |            |            |            |            |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
| ✓           | ✓         |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

Game Maker Studio is commercial software, regulated by its own EULA, but it was added here for sake of completeness.

### GDevelop {.unnumbered .unlisted}

**Website:** <https://gdevelop-app.com/>

**Price:** Free

GDevelop is an open-source toolkit to make games, mostly based on visual programming, GDevelop supports the use of JavaScript to code parts of the game, as well as JSON and support for HTTP Requests. GDevelop also supports exporting to Android and Facebook Instant Games, as well as exporting to iOS and web-based platforms.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            |            |            |            |            |            |            |            |            | ✓ |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      | ✓         |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

GDevelop is distributed under the MIT license (although the name and logo are copyright), although the main license file refers to other license files inside each folder. So you may want to check the GitHub repository for more information.

### GLFW {.unnumbered .unlisted}

**Website:** <https://www.glfw.org/>

**Price:** Free

GLFW is an Open-Source library for OpenGL, OpenGL ES and Vulkan, that allows to create windows, context and surfaces, as well as receiving input and events. It is written in C.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ |            |            |            |

GLFW is distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.

### Godot {.unnumbered .unlisted}

**Website:** <https://godotengine.org/>

**Price:** Free

Godot is a fully-fledged engine that makes use of a node-based approach and supports many ways of programming your own game, both in 2D and 3D, including its own language (GDScript) and visual scripting.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓ |            | ✓ |            |            |            |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
| ✓           | ✓         |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

Godot is currently distributed under the MIT license, you should check the Legal section of the Godot Documentation for all the additional licenses that you may need to know about.

### Gosu {.unnumbered .unlisted}

<https://www.libgosu.org/>

{{placeholder}}

### IRRlicht {.unnumbered .unlisted}

**Website:** <http://irrlicht.sourceforge.net/>

**Price:** Free

IRRlicht is a 3D engine (as in does only 3D rendering) made in C++ that aims to be high-performance.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓ |            | ✓ |            | ✓ | ✓ | ✓ |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ |            |            |            |

IRRlicht distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.

### Löve {.unnumbered .unlisted}

**Website:** <http://love2d.org/>

**Price:** Free

Löve is a lua-based framework for creating games, features an extensive documentation and features some levels of abstraction that help with game development.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            |            |            |            |            |            |            | ✓ |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ |            |

Löve is distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.

### MonoGame {.unnumbered .unlisted}

**Website:** <http://www.monogame.net/>

**Price:** Free

MonoGame is an open-source porting of XNA 4, it allows for people used to Microsoft's XNA framework to port their games to other platforms, as well as creating new games from scratch. Many games make use of this framework, one above all: Stardew Valley.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            |            | ✓ |            |            |            |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ |            |

MonoGame is distributed under a mixed license: Microsoft Public License + MIT License. You may want to check the license yourself on the project's GitHub page.

### olcPixelGameEngine {.unnumbered .unlisted}

**Website:** <https://github.com/OneLoneCoder/olcPixelGameEngine>

**Price:** Free

A small framework for pixel drawing and user interface framework coded by javidx9 (also known as OneLoneCoder), it is made up of a single header file, making it extremely lightweight but also rich in features.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓          |            | ✓          |            | ✓          |            |            | ✓          | ✓          |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓          | ✓          | ✓          |            |            |            |

### Ogre3D {.unnumbered .unlisted}

**Website:** <https://www.ogre3d.org/>

**Price:** Free

Ogre3D is an open source 3D graphics engine (it's used to render 3D graphics only).

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓          |            | ✓          |            | ✓          | ✓          | ✓          |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓          | ✓          | ✓          |            |            |            |

Ogre3D comes in 2 versions: version 1.x is distributed under the GNU LGPL license, while the more recent 2.x version is distributed under the more premissive MIT license.

### Panda3D {.unnumbered .unlisted}

**Website:** <https://www.panda3d.org/>

**Price:** Free

Panda3D is a complete and open source 3D game engine.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓ |            |            |            |            | ✓ |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ |            |            |            |

Panda3D itself is distributed under the modified BSD license, which is very permissive, but it brings together many third-party libraries that are released under different licenses. It is suggested to check the license section of the manual for more information.

### SDL {.unnumbered .unlisted}

**Website:** <https://www.libsdl.org/>

**Price:** Free

SDL (Simple DirectMedia Layer) is one of the most famous libraries around to make multimedia applications as well as videogames.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓ | ✓ | ✓ | ✓ |            | ✓ |            | ✓ | ✓ |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ |            |

SDL is distributed under the ZLib license, which allows for both commercial and personal use, both in proprietary and open-source situations. Many of the languages listed as "usable" are compatible via extensions.

The versions of SDL up to version 1.2 are instead distributed under the GNU LGPL license, which is more complex and may need to be analyzed by legal experts.

### SFML {.unnumbered .unlisted}

**Website:** <https://www.sfml-dev.org/>

**Price:** Free

SFML (Simple Fast Multimedia Library) is a library dedicated to creation of multimedia applications (not limited to videogames), providing a simple interface to the system's components.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |            | ✓ |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | *          | *          |            |

\*: Currently in development

SFML is distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.

### Unity 3D {.unnumbered .unlisted}

**Website:** <https://unity.com/>

**Price:** Free for Personal Use + Paid Version for commercial projects

Unity is probably among the most famous 3D engines used to create videogames, as well as other projects that make use of its 3D capabilities (like VR/AR-based projects). It uses the C# programming language.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            |            | ✓ |            |            |            |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

Unity is a proprietary engine, distributed under a proprietary license, with a Free edition available.

### UPBGE {.unnumbered .unlisted}

**Website:** <https://upbge.org/>

**Price:** Free

Since 2018, Blender doesn't ship with its own game engine anymore. UPBGE is a project to bring back such feature, built on top of the newer Blender versions, and then some.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            |            |            |            |            |  ✓         |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|  ✓                   |  ✓                 |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ |   |   |   |


Genre-Specific
--------------

Sometimes you don't need all the tools to make your own game, sometimes you want your adventure to fit some pretty common standards and not deviate too much from them. Genre-specific engines allow you to ditch both the complexity and freedom of the general-purpose engines to have an easier time making your own adventure.

These engines allow you to focus more on the content of your game, instead of wasting time crunching code.

### EasyRPG {.unnumbered .unlisted}

**Website:** <https://easyrpg.org/>

**Price:** Open Source

EasyRPG is a development environment focused on creating RPG games, trying to maintain compatibility with the older RPG Maker 2000/2003 software.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            |            |            |            |            |            |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
| ✓           | ✓         |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

### O.H.R.RPG.C.E. {.unnumbered .unlisted}

{{placeholder}}

<!--TODO: Description -->

<http://rpg.hamsterrepublic.com/ohrrpgce/About>

### OpenBOR {.unnumbered .unlisted}

**Website:** <https://github.com/DCurrent/openbor>

**Price:** Open Source

OpenBOR is an open source implementation of a side-scroller beat-em-up engine (in the style of Streets of Rage or Final Fight). It has an active community, even if releases are infrequent.

**Relevant Bindings:**

| C++        | C          | C#         | Go         | Java       | Python     | Ruby       | Lua        | Rust       | JavaScript |
|:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |:-:         |
|            | ✓          |            |            |            |            |            |            |            |            |

**Other Bindings:**

| Proprietary Language | Visual Programming |
| :-:                  | :-:                |
|                      |                    |

**Platform Compatibility:**

| Windows    | Linux      | Mac OS     | iOS        | Android    | Web-Based  |
|:-------:   |:-----:     |:------:    |:---:       |:-------:   |:---------: |
| ✓          | ✓          |            |            | ✓          |            |

### Ren'Py {.unnumbered .unlisted}

{{placeholder}}

<!--TODO: Description -->

<https://www.renpy.org/>

### RPG Maker {.unnumbered .unlisted}

{{placeholder}}

<!--TODO: Description -->

<https://www.rpgmakerweb.com/>

### RPG Paper Maker {.unnumbered .unlisted}

{{placeholder}}

<!--TODO: Description -->

<http://rpg-paper-maker.com/>


### Stratagus {.unnumbered .unlisted}

{{placeholder}}

<!--TODO: Description -->

<https://github.com/Wargus/stratagus>

<!-- TODO: Here go all the specific engines, like RPG maker -->
