\null\clearpage

Useful Patterns, Containers and Classes
========================================

In this chapter we will introduce some useful design patterns, data containers and classes that could help you solve some issues or increase your game's maintainability and flexibility.

Design Patterns
----------------

Design Patterns are essentially "pre-made solutions for known problems" and can help decoupling elements of your game, increasing maintainability.

Obviously design patterns are not a cure-all, they can introduce overhead and could lead to over-engineering: balance is key when it comes to creating a game (or any software in general).

### Singleton Pattern

Sometimes it can be necessary to ensure that there is one and only instance of a certain object across the whole program, this is where the *singleton* design pattern comes into play.

To make a singleton, it is necessary to hide (make private) the class constructor, so that the class itself cannot be instantiated via its constructor.

After that, we need a static method that allows to get the singleton's instance, the method needs to be static to make it callable without an instance of the singleton class.

The UML diagram for a singleton is really simple.

![The UML diagram for a singleton pattern](./images/patterns_containers/singleton.png){width=20%}

\code{patterns_containers/singleton}{Example of a singleton pattern}

The previous singleton instantiates immediately, which may not always be necessary, in that case a good idea could be implementing the so-called "lazy loading", where the instantiation happens the first time you ask the object for its own instance.

\code{patterns_containers/singleton_lazyload}{Example of a singleton pattern with lazy loading}

If multiple threads are involved in using a lazy-loading singleton, you may need to take care of preventing *race conditions*~[g]~ that could result in multiple instances of the singleton being created.

Many critics consider the singleton to be an "anti-pattern", mostly because it is really overused and adds a possibly unwanted "global state" (see it as a global variable, but in an object-oriented sense) into the application.

Before applying the singleton pattern, ask yourself the following questions:

- Do I really need to **ensure** that only one instance of this object is present in the whole program?
- Would the presence of more than one instance of this object be detrimental to the functionality of this program? How?
- Does the instance of this object **need** to be accessible from everywhere in the program?

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Singleton

**When to Use it**    In all situations that strictly require one instance of an object, accessible globally.

**Advantages **       Allows the class to control its own instantiation, allows for easier access to the sole instance of a class.

**Disadvantages**     Introduces some restrictions that may be unnecessary, introduces a global state into the application.
------------------------------------------------------------------------------------------------


### Command Pattern

It may be necessary, during our software development, to abstract our functions into something that can be assigned and treated as an object.

Many programming languages now feature functions as "first class citizens", allowing to treat functions as objects: assigning functions to variables, calling functions, lambdas, inline functions, functors, function pointers...

The command pattern allows us to abstract a function (or any executable line of code) into its own object that can be handled as such, allowing us to package a request into its own object for later use.

This pattern can be useful to code GUIs, making actions in our games that can be undone, macros, replays and much more.

\placeholder

<!-- TODO: Talk about the command pattern and how to transform a function into an object -->
<!-- TODO: Add pattern reference table -->

### Flyweight

Sometimes it may be necessary to keep track of a large number of very similar objects.

Imagine a lot of sprites of trees that have the same texture and size, but have different positions: it could prove to be really resource-heavy to keep all the sprite objects in memory, each one with its own copy of the texture and size. This could prove to be performance-heavy too, since all those textures will have to be moved to the GPU.

Here comes the Flyweight pattern: we try to share as much of the so-called "intrinsic state" of the objects between the object that contain the so-called "extrinsic state".

\placeholder

<!-- TODO: Extracting intrinsic stuff in a separate class and use lightweight classes for extrinsic stuff (intrinsic: rock meshes and textures, extrinsic: rock positions and orientations) -->
<!-- TODO: Add pattern reference table -->

### Observer Pattern {#ObserverPattern}

The observer pattern is used to implement custom event handling systems, where an object automatically reacts to events generated by another object.

There are 2 main objects used in an observer pattern:

- **The Subject**: sometimes called "Observed Object"
- **The observer**: sometimes called "Dependent Object"

The subject is the creator of a "stream of events" that is consumed by the observer objects.

The subject implements in its structure a list of observers that will be notified when a change occurs, as well as methods to register (add) a new observer as well as to unregister (remove) an existing observer, while the observers will implement a method that will be called by the subject, so that the observers can be notified of such change.

Here we can see an UML diagram of the observer pattern:

![The UML diagram of the observer pattern](./images/patterns_containers/observer.png){width=60%}

Here we can see the Observer abstract class (it can be an interface), a concrete subject and two Concrete Observers that implement what required by the Observer.

Here we can see an implementation of the observer pattern:

\code{patterns_containers/observer}{Code for an observer pattern}

If needed, you can pass information between the subject and the observers just by calling each `update()` method with the necessary arguments.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Observer

**When to Use it**    Event Handling systems, making objects react to other objects' actions

**Advantages **       Decoupling, added flexibility, more performant than if statements for conditions that happen rarely.

**Disadvantages**     Can be a bit hard to set up, makes the architecture more complex, if un-registration is not done well there could be serious memory leaks (even in garbage-collected languages).
------------------------------------------------------------------------------------------------

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

Menu Stack
-----------

Although menus can be represented via a finite state machine, the structure of an User Interface (UI) is better suited for another data model: the stack (or rather, something similar to a stack).

A stack allows us to code some other functions in an easier way, for instance we can code the "previous menu" function by just popping the current menu out of the stack; when we access a new menu, we just push it into the menu stack and the menu stack will take care of the rest.

Unlike the stacks we are used to, the menu stack can also be accessed like a queue (first in - first out) so you can draw menus and dialogs on top of each other, while the last UI element (on top of the stack) keeps the control of the input-update-draw cycle.

In the menu stack we also have some functionalities that may not be included in a standard stack, like a "clear" function, which allows us to completely clean the stack: this can prove useful when we are accessing the main game, since we may not want to render the menu "below" the main game, wasting precious CPU cycles.

\placeholder

<!-- TODO: Make a structure with a menu stack with "UI Elements" that can refer to the stack itself (for the "back" action for instance), add diagram and code -->

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

\code{patterns_containers/tween_linear}{Linear Tweening}

Let's explain the variables used:

- **time**: The current time of the tween. This can be any unit (frames, seconds, steps, ...), as long as it is the same unit as the "duration" variable;
- **begin**: represents the beginning value of the property being inbetweened;
- **change**: represents the change between the beginning and destination value of the property;
- **duration**: represents the duration of the tween.

Note that the measure (time / duration) represents the "percentage of completion" of the tweening.

In some cases a Linear tweening is not enough, that's where *easing* comes into play.

Before introducing easing let's analyze the function again, if you try plugging in some data into the function, you will find that there is always going to be:

$$ (change\ in\ property) \cdot (factor) + (beginning\ value)$$

So we can use our function substituting `begin` with 0 and `change` with 1 to calculate `factor` and have a code similar to this one:

\code{patterns_containers/way_to_easing}{Example of a simple easing function}

With linear tweening, the function degenerates to $\frac{time}{duration}$, but now we can replace our linear tween with the following function:

\code{patterns_containers/easeIn}{Ease-in}

By changing the `power` parameter, we change the behaviour of the easing, making the movement slower at the beginning and pick up the pace more and more, until the destination is reached. This is called a "ease-in".

For an "ease-out", where the animation starts fast and slows down towards the end, we use the following function instead:

\code{patterns_containers/easeOut}{Ease-out}

With some calculations, and if statements on the time passed, you can combine the two and get an "ease-in-out" function.

Obviously these functions have an issue: they don't clamp the value between 0 and 1, that will have to be done in the movement function or by adding a check, or using some math, for instance using `min(calculated_value, 1)`.

\placeholder

<!-- TODO: Also known as "tweening", allows to "smear" a value over time, usually used to generate key frames in an animation to make it seem like it moves smoothly over time, while in reality you just set the beginning and end positions, along with the time the movement should take -->

Chaining
--------

\placeholder

<!-- TODO: An abstraction that allows to chain operations one after another, usually said operations are timed or related to tweening -->
