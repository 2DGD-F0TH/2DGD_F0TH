Introduction
=============

Welcome to the book! This book aims to be an organized collection of the community's knowledge on game development techniques, algorithms and experience with the objective of being as comprehensive as possible.

It's really common in today's game development scene to approach game development through tools that abstract and guide our efforts, without exposing us to the nitty-gritty details of how things work on low-level and speeding up and easing our development process. This approach is great when things work well, but it can be seriously detrimental when we are facing against issues: we are tied to what the library/framework creators decided was the best (read "applicable in the widest range of problems") approach to solving a problem.

Games normally run at 30fps, more modern games run at 60fps, some even more, leaving us with between 33ms to 16ms or less to process a frame, which includes:

- Process the user input;
- Update the player movement according to the input;
- Update the state of any AI that is used in the level;
- Move the NPCs according to their AI;
- Identify Collisions between all game objects;
- React to said Collisions;
- Update the Camera (if present);
- Update the HUD (if present);
- Draw the scene to the screen.

These are only some basic things that can be subject to change, **in a single frame**.

When things don't go well, the game lags, slows down or even locks up. In that case we will be forced to take the matter in our hands and get dirty handling things exactly as we want them (instead of trying to solve a generic problem).

This book wants to open the box that contains everything related to 2D game development, plus some small tips and tricks to make your game more enjoyable. This way, if your game encounters some issues, you won't fear diving into low-level details and fix it yourself.

Or why not, make everything from scratch using some pure-multimedia interfaces (like SDL or SFML).

Enjoy!

Conventions used in this book
-----------

### Logic Conventions

When talking about logic theory, the variables will be represented with a single uppercase letter, written in math mode: $A$

The following symbol will be used to represent a logical "AND": $\land$

The following symbol will be used to represent a logical "OR": $\lor$

The logical negation of a variable will be represented with a straight line on top of the variable, so the negation of the variable $A$ will be $\bar{A}$

### Code Listings

Code listings will be shown in monotype fonts, using syntax highlighting where possible:

\code{introduction/example}{Example code listing}

<!-- TODO: Insert more conventions, assumptions... -->
