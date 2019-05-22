\null\clearpage
\appendix
\addtocontents{toc}{\protect\setcounter{tocdepth}{1}}
\setcounter{secnumdepth}{0}
Engines, Libraries And Frameworks
=================================

Here follows a list of game engines, libraries and frameworks, complete with their known language bindings, operating system compatibility and a short description.

SFML
-----

SFML (Simple Fast Multimedia Library) is a library dedicated to creation of multimedia applications (not limited to videogames), providing a simple interface to the system's components.

Relevant Bindings:

| C++ | C | .Net | Go | Java | Python | Ruby | Lua | Rust |
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

SDL (Simple DirectMedia Layer) is one of the most famous libraries around to make multimedia applications as well as videogames.

Relevant Bindings:

| C++ | C | .Net | Go | Java | Python | Ruby | Lua | Rust |
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

Löve is a lua-based framework for creating games, features an extensive documentation and features some levels of abstraction that help with game development.

Relevant Bindings:

| C++ | C | .Net | Go | Java | Python | Ruby | Lua | Rust |
|:---:|:-:|:----:|:--:|:----:|:------:|:----:|:---:|:----:|
|     |   |      |    |      |        |      | \checkmark |  |

Operating System Compatibility:

| Windows | Linux | Mac OS | iOS | Android |
|:-------:|:-----:|:------:|:---:|:-------:|
|   \checkmark     | \checkmark     |  \checkmark     |  \checkmark  |  \checkmark      |

Löve is distributed under the ZLib/png license, which allows for both commercial and personal use, both in proprietary and open-source situations.
