\null\clearpage
Engines, Libraries And Frameworks
=================================

Here follows a list of game engines, libraries and frameworks, complete with their known language bindings, operating system compatibility and a short description.

SFML
-----

<https://www.sfml-dev.org/>

SFML (Simple Fast Multimedia Library) is a library dedicated to creation of multimedia applications (not limited to videogames), providing a simple interface to the system's components.

Relevant Bindings:

| C++ | C | C# | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
|  \checkmark  | \checkmark |  \checkmark   | \checkmark  | \checkmark    | \checkmark      | \checkmark    |     | \checkmark    |

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  *  |  *      |

\*: Currently in development

SFML is distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.

SDL
-------

<https://www.libsdl.org/>

SDL (Simple DirectMedia Layer) is one of the most famous libraries around to make multimedia applications as well as videogames.

Relevant Bindings:

| C++ | C | C# | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
| \checkmark | \checkmark | ^1^ | ^2^ | | ^3^ | | ^4^ | ^5^ |

- ^1^ : Via SDL2-CS
- ^2^ : Via go-sdl2
- ^3^ : Via Pygame, Py-SDL2 or pysdl2-cffi
- ^4^ : Via Lua-SDL2
- ^5^ : Via Rust-SDL2

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  \checkmark  |  \checkmark      |

SDL is distributed under the ZLib license, which allows for both commercial and personal use, both in proprietary and open-source situations.

The versions of SDL up to version 1.2 are instead distributed under the GNU LGPL license, which is more complex and requires source code disclosure.

Löve
-----

<http://love2d.org/>

Löve is a lua-based framework for creating games, features an extensive documentation and features some levels of abstraction that help with game development.

Relevant Bindings:

| C++ | C | C# | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
|     |   |      |    |      |        |      | \checkmark |  |

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  \checkmark  |  \checkmark      |

Löve is distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.

Unity 3D
--------

<https://unity.com/>

Unity is probably among the most famous 3D engines used to create videogames, as well as other projects that make use of its 3D capabilities (like VR/AR-based projects). It uses the C# programming language.

Relevant Bindings:

| C++ | C | C# | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
|     |   |  \checkmark    |    |      |        |      |  |  |

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  \checkmark  |  \checkmark      |

Unity is a proprietary engine, distributed under a proprietary license, with a Free edition available.


IRRlicht
--------

<http://irrlicht.sourceforge.net/>

IRRlicht is a 3D engine (as in does only 3D rendering) made in C++ that aims to be high-performance.

Relevant Bindings:

| C++ | C | C# | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
| \checkmark |   | \checkmark  |    |  \checkmark | \checkmark  | \checkmark |  |  |

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  |  |

IRRlicht distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.


Ogre3D
------

<https://www.ogre3d.org/>

Ogre3D is an open source 3D graphics engine (it's used to render 3D graphics only).

Relevant Bindings:

| C++ | C | C# | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
| \checkmark |   | \checkmark  |    |  \checkmark | \checkmark  | \checkmark |  |  |

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  |  |

Ogre3D comes in 2 versions: version 1.x is distributed under the GNU LGPL license, while the more recent 2.x version is distributed under the more premissive MIT license.

Panda3D
---------

<https://www.panda3d.org/>

Panda3D is a complete and open source 3D game engine.

Relevant Bindings:

| C++ | C | C# | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
| \checkmark |   | |    |  | \checkmark  | |  |  |

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  |  |

Panda3D itself is distributed under the modified BSD license, which is very permissive, but it brings together many third-party libraries that are released under different licenses. It is suggested to check the license section of the manual for more information.
