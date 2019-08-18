\null\clearpage
Useful Patterns, Containers and Classes
========================================

Resource Manager
-----------------

\placeholder

<!-- TODO: An associative container that contains pairs (ENUM, ITEM) used to store and retrieve
objects for the game -->

Animator
---------

\placeholder

<!-- TODO: A class that yelds frames, with an internal counter and all the necessary facilitations for
animating a sprite -->

Finite State Machine
---------------------

A finite state machine is a model of computation that represents an abstract machine with a finite number of possible states but where one (or a finite number) of states can be "in execution" at a given time.

We can use a finite state machine to represent the status of a player character, like in the following diagram:

![Diagram of a character's state machine](./images/patterns_containers/Character_SM.pdf){width=70%}

Each state machine is made out of two main elements:

- **states** which define a certain state of the system (for the diagram above, the states are: Idle, Crouching, Jumping and Stomping);
- **transitions** which define a condition and the change of state of the machine (for the diagram above there are two "Press Down" transitions, one "Release Down" and one "Press Space")

State machines are really flexible and can be used to represent a menu system, for instance:

![Diagram of a menu system's state machine](./images/patterns_containers/Menu_SM.pdf){width=70%}

In this more convoluted diagram we can see how pressing a certain button or clicking a certain option can trigger a state change.

Each state can be created so it has its own member variables and methods: in a menu system it can prove useful to have each state have its own `update(dt)` and `draw()` functions to be called from the main game loop, to improve on the code readability and better usage of the nutshell programming principle.

\placeholder

<!-- TODO: A simple finite state machine that allows to change states, useful for menus and stuff -->

Particle Systems
-----------------

\placeholder

<!-- TODO: Talk about particle systems, particles and particle emitters -->

Timers
------

\placeholder

<!-- TODO: A timer class that allows to execute a certain instruction every x seconds, abstracting the concept of frames -->

Inbetweening
--------

Inbetweening, also known as "tweening", is a method that allows to "smear" a value over time, this is usually done with animations, where you set the beginning and end position of a certain object, as well as the time the movement should take, and let the program take care of the animation.

This is particularly useful in animating *UI*~[g]~ objects, to give a more refined feel to the game.

Here we will present some simple tweenings that can be programmed, and explain them.

Let's start with a *linear* tweening, usually the following function is used:

\code{patterns_containers/tween_linear}

Let's explain the variables used:

- **time**: The current time of the tween. This can be any unit (frames, seconds, steps, ...), as long as it is the same unit as the "duration" variable;
- **begin**: represents the beginning value of the property being inbetweened;
- **change**: represents the change between the beginning and destination value of the property;
- **duration**: represents the duration of the tween.

Note that the measure (time / duration) represents the "percentage of completion" of the tweening.

<!-- TODO: Also known as "tweening", allows to "smear" a value over time, usually used to generate key frames in an animation to make it seem like it moves smoothly over time, while in reality you just set the beginning and end positions, along with the time the movement should take -->

Chaining
--------

\placeholder

<!-- TODO: An abstraction that allows to chain operations one after another, usually said operations are timed or related to tweening -->
