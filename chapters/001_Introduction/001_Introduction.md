{{pagebreak}}

Introduction
=============

:::::: {.epigraph author="Laozi - Tao Te Ching"}
A journey of a thousand miles begins with a single step
::::::::::::::::

Welcome to the book! This book aims to be an organized collection of the community's knowledge on game development techniques, algorithms and experience with the objective of being as comprehensive as possible.

Why another game development book?
----------------------------------

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

These are only some basic things that can be subject to change in a game, **every single frame**.

When things don't go well, the game lags, slows down or even locks up. In that case we will be forced to take the matter in our hands and get dirty handling things exactly as we want them (instead of trying to solve a generic problem).

When you are coding a game for any device that doesn't really have "infinite memory", like a mobile phone, consoles or older computers, this "technical low-level know-how" becomes all the more important.

This book wants to open the box that contains everything related to 2D game development, plus some small tips and tricks to make your game more enjoyable. This way, if your game encounters some issues, you won't fear diving into low-level details and fix it yourself.

Or why not, make everything from scratch using some pure-multimedia interfaces (like SDL or SFML) instead of fully fledged game engines (like Unity).

This book aims to be a free (as in price) teaching and reference resource for anyone who wants to learn 2D game development, including the nitty-gritty details.

Enjoy!

Conventions used in this book
-----------

### Logic Conventions

When talking about logic theory, the variables will be represented with a single uppercase letter, written in math mode: $A$

The following symbol will be used to represent a logical "AND": $\land$

The following symbol will be used to represent a logical "OR": $\lor$

The logical negation of a variable will be represented with a straight line on top of the variable, so the negation of the variable $A$ will be $\bar{A}$

### Code Listings

Listings, algorithms and anything that is code will be shown in monotype fonts, using syntax highlighting where possible, inside of a dedicated frame:

```{src="introduction/example" caption="Example code listing"}
```

### Block Quotes

There will be times when it's needed to write down something from another source verbatim, for that we will use block quotes, which are styled as follows:

> Hi, I'm a block quote! You will see me when something is... quoted!
>
> I am another row of the block quote! Have a nice day!

### Boxes

In your journey through this book, you may find some boxes, let's see which ones you may come across.

:::: tip ::::
This is a tip box, here you will find tips that are loosely related to the chapter at hand. These small tips will help you make a better game, or wiggle your way through something difficult.
::::

:::: pitfall ::::
This is a pitfall box, it will warn you of traps behind the corner, as well as possible shortcomings of a certain solution.
::::

:::: trivia ::::
This is a trivia box, it will give out some small facts that can help you understand things better, or just give you a small break from all the learning.
::::

:::: note ::::
This is just a note box, it's not a pitfall, a tip or a trivia. This is used for reminders and just as a general purpose note
::::::::::::::

### Engine Used

This book does not use any engine. All algorithms will be presented pretending there is some "generic engine" behind the scenes that handles sprites, vectors and the like. The objective of this book is teaching algorithms, tips and tricks and game design in the most engine-agnostic (and language-agnostic, if you're looking at the "pseudocode edition") way possible.

### About editions

This book comes in various editions, and they come with some caveats.

- **Pseudocode Edition:** This is the standard edition, using a C-like syntax that tries to be as readable as possible and abstracts itself from any kind of engine;
- **Python Edition:** Python is considered one of the easiest language to start coding on. Many tend to complain about its performance, but its similarity to Godot Engine's GDScript and its flexibility make it a good candidate for starters.
- **C++ Edition:** C++ is probably the most used language in game development (along with C#) but it can be really difficult to manage. It has no garbage collection, forcing you to manage the memory manually, and pointers can prove to be a difficult concept for many.
- **JavaScript Edition:** Javascript is the de-facto "internet language" and its influence is spreading to desktop applications and videogames too. Many games now can be played on the browser thanks to it and the HTML5 canvas elements. This is a language that can be very forgiving and frustrating at the same time.
<!-- TODO: When Lua edition is out, uncomment me!
- **Lua Edition:** Lua is one of the most spread scripting languages in the world of videogames. Having a very small interpreter it can be added to a lot of code bases without weighing them down much. It is not a proper object-oriented language, but it has very strong metaprogramming capabilities (where you can "program the programming language"). There are some libraries that allow for classes and object-oriented concepts to fit in Lua.
-->

<!-- TODO: Insert more conventions, assumptions... -->

Structure of this Book
----------------------

This book is structured in many chapters, here you will find a small description of each and every one of them.

- **Foreword:** You didn't skip it, right?
- **Introduction:** Here we present the structure of the book and the reasons why it came to exist. You are reading it now, hold tight, you're almost there!
- **The Maths Behind Game Development:** Here we will learn the basic maths that are behind any game, like vectors, matrices and screen coordinates.
- **Some Computer Science Fundamentals:** Here we will learn (or revise) some known computer science fundamentals (and some less-known too!) and rules that will help us managing the development of our game.
- **Project Management Tips:** Project management is hard! Here we will take a look at some common pitfalls and tips that will help us deliver our own project and deliver it in time.
- **Introduction to game design:** In this section we will talk about platforms games can run on, input methods as well as some game genres.
- **Writing a Game Design Document:** In this section we will take a look at one of the first documents that comes to exist when we want to make a game, and how to write one.
- **The Game Loop:** Here we will learn the basics of the "game loop", the very base of any video game.
- **Collision Detection and Reaction:** In this section we will talk about one of the most complex and computationally expensive operations in a video game: collision detection.
- **Cameras:** In this section we will talk about the different types of cameras you can implement in a 2D game, with in-depth analysis and explanation;
- **Game Design:** In this chapter we will talk about level design and how to walk your player through the learning and reinforcement of game mechanics, dipping our toes into the huge topic that is game design.
- **Creating your resources:** Small or solo game developers may need to create their own resources, in this section we will take a look at how to create our own graphics, sounds and music.
- **Procedural Content Generation:** In this chapters we will see the difference between procedural and random content generation and how procedural generation can apply to more things than we think.
- **Design Patterns:** A head-first dive into the software engineering side of game development, in this section we will check many software design patterns used in many games;
- **Useful Containers and Class:** A series of useful classes and containers used to make your game more maintainable and better performing.
- **Artificial Intelligence in Video games:** In this section we will talk about algorithms that will help you coding your enemy AI, as well as anything that must have a "semblance of intelligence" in your video game;
- **Other Useful Algorithms:** In this section we will see some algorithms that are commonly used in game, including path finding, world generation and more.
- **Developing Game Mechanics:** Here we will dive into the game development's darkest and dirtiest secrets, how games fool us into strong emotions but also how some of the most used mechanics are implemented.
- **Accessibility in video games:** Here we will learn the concept of "accessibility" and see what options we can give to our players to make our game more accessible (as well as more enjoyable to use).
- **Testing your game:** This section is all about hunting bugs, without a can of bug spray. A deep dive into the world of testing, both automated and manual.
- **Optimizing and Profiling your game:** When things don't go right, like the game is stuttering or too slow, we have to rely on profiling and optimization. In this section we will learn tips and tricks and procedures to see how to make our games perform better.
- **Balancing Your Game:** A very idealistic vision on game balance, in this chapter we will take a look inside the player's mind and look at how something that may seem "a nice challenge" to us can translate into a "terrible balance issue" to our players.
- **Marketing Your Game:** Here we will take a look at mistakes the industry has done when marketing and maintaining their own products, from the point of view of a small indie developer. We will also check some of the more controversial topics like loot boxes, micro transactions and season passes.
- **Engaging your community:** a lot of a game's power comes from its community, in this section we will take a look at some suggestion you can implement in your game (and out-of-game too) to further engage your loyal fans.
- **Game Jams:** A small section dedicated on Game Jams and how to participate to one without losing your mind in the process, and still deliver a prototype.
- **Dissecting Games:** A small section dedicated to dissecting the characteristics of one (very) bad game, and one (very) good game, to give us more perspective on what makes a good game "good" and what instead makes a bad one.
- **Project Ideas:** In this section we take a look at some projects you can try and make by yourself, each project is divided into 3 levels and each level will list the skills you need to master in order to be able to take on such level.
- **Where to go from here:** We're at the home stretch, you learned a lot so far, here you will find pointers to other resources that may be useful to learn even more.
- **Glossary:** Any world that has a ~[g]~ symbol will find a definition here.
- **Engines and Frameworks:** A collection of frameworks and engines you can choose from to begin your game development.
- **Tools:** Some software and tool kits you can use to create your own resources, maps and overall make your development process easier and more manageable.
- **Premade Assets and resources:** In this appendix we will find links to many websites and resource for graphics, sounds, music or learning.
- **Contributors:** Last but not least, the names of the people who contributed in making this book.

Have a nice stay and let's go!
