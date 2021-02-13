\null\clearpage

Useful Patterns, Containers and Classes
========================================

\epigraph{Eliminate effects between unrelated things. Design components that are self-contained, independent, and have a single, well-defined purpose.}{\textit{Anonymous}}

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

![The UML diagram for a singleton pattern](./images/patterns_containers/singleton.pdf){width=20%}

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

Table: Summary table for the Singleton Pattern


### Command Pattern

It may be necessary, during our software development, to abstract our functions into something that can be assigned and treated as an object.

Many programming languages now feature functions as "first class citizens", allowing to treat functions as objects: assigning functions to variables, calling functions, lambdas, inline functions, functors, function pointers...

The command pattern allows us to abstract a function (or any executable line of code) into its own object that can be handled as such, allowing us to package a request into its own object for later use.

This pattern can be useful to code GUIs, making actions in our games that can be undone, macros, replays and much more.

![UML diagram for the Command Pattern](./images/patterns_containers/command.pdf){width=30%}

\code{patterns_containers/command}{Example code for the Command Pattern}

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Command

**When to Use it**    In all situations where you want to avoid coupling an invoker with a single request or when you want to configure an invoker to perform a request at runtime.

**Advantages **       Allows for encapsulation, less coupling, more flexibility and customization at runtime.

**Disadvantages**     Late binding and objects may introduce some overhead.

------------------------------------------------------------------------------------------------

### Flyweight

Sometimes it may be necessary to keep track of a large number of very similar objects.

Imagine a lot of sprites of trees that have the same texture and size, but have different positions: it could prove to be really resource-heavy to keep all the sprite objects in memory, each one with its own copy of the texture and size. This could prove to be performance-heavy too, since all those textures will have to be moved to the GPU.

Here comes the Flyweight pattern: we try to share as much of the so-called "intrinsic state" of the objects between the object that contain the so-called "extrinsic state".

![UML Diagram of the Flyweight pattern](./images/patterns_containers/flyweight.pdf){width=60%}

Below is an example code for the flyweight pattern.

\code{patterns_containers/flyweight}{Code for a flyweight pattern}

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Flyweight

**When to Use it**    When you need to support a large number of similar objects efficiently, when you need to avoid creating a large number of objects.

**Advantages **       Saves memory when a large number of similar objects is involved, avoids some of the overhead given by the creation of many objects.

**Disadvantages**     The intrinsic state must be "context independent", so it cannot change (or all the flyweights that refer to that state will change too). Flyweight instantiation requires particular attention in multithreaded environments, due to the shared memory.

------------------------------------------------------------------------------------------------

Table: Summary table for the Flyweight Pattern

### Observer Pattern {#ObserverPattern}

The observer pattern is used to implement custom event handling systems, where an object automatically reacts to events generated by another object.

There are 2 main objects used in an observer pattern:

- **The Subject**: sometimes called "Observed Object"
- **The observer**: sometimes called "Dependent Object"

The subject is the creator of a "stream of events" that is consumed by the observer objects.

The subject implements in its structure a list of observers that will be notified when a change occurs, as well as methods to register (add) a new observer as well as to unregister (remove) an existing observer, while the observers will implement a method that will be called by the subject, so that the observers can be notified of such change.

Here we can see an UML diagram of the observer pattern:

![The UML diagram of the observer pattern](./images/patterns_containers/observer.pdf){width=60%}

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

Table: Summary table for the Observer Pattern

### Strategy

In some situations it may be necessary to select a single algorithm to use, from a family of algorithms, and that decision must happen at runtime.

In this case, the *strategy pattern* (also knowns as the "policy pattern"), allows the code to receive runtime instructions over what algorithm to execute. This allows for the algorithm to vary independently from the client that makes use of such algorithm.

![The UML diagram of the strategy pattern](./images/patterns_containers/strategy.pdf){width=60%}

\code{patterns_containers/strategy}{Code for a strategy pattern}

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Strategy

**When to Use it**    Every time you need to decide which algorithm to execute at runtime.

**Advantages **       Decoupling, added flexibility.

**Disadvantages**     Can cause proliferation of similarly-looking concrete strategies, late binding on functions and the object oriented nature of the pattern could create some overhead.

------------------------------------------------------------------------------------------------

Table: Summary table for the Strategy Pattern

### Chain of Responsibility

Sometimes we have the necessity of handling conditionals that are themselves connected to runtime conditions. This is where the *chain of responsibility pattern* comes into play, being essentially an object-oriented version of an `if ... else if ... else` statement.

![UML Diagram of the Chain of Responsibility Pattern](./images/patterns_containers/chain_of_responsibility.pdf){width=60%}

As can be seen from the diagram, the sender is not directly connected to the receiver, but instead it's connected to a "Handler" interface, making them independent.

As with a chain of responsibility in a company relays a task to "higher ups" if the task cannot be handled, the chain of responsibility pattern involves each received reviewing the request and if possible, process it, if not possible, relay it to the next receiver in the chain.

\code{patterns_containers/chain_of_responsibility}{Code for a chain of responsibility pattern}

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Chain of Responsibility

**When to Use it**    When you need to implement flexible if...else if...else statements that change on runtime. When you want to decouple a sender from a receiver.

**Advantages **       Decoupling, added flexibility.

**Disadvantages**     Some overhead is added by the objects and late binding, could lead to proliferation of similar-looking handlers/receivers.

------------------------------------------------------------------------------------------------

Table: Summary table for the Chain of Responsibility Pattern

### Component/Composite Pattern

When building any game entity, we find that the complexity of the game entity itself literally explodes: a monolithic class can include loads of different operations that should stay separate, such as:

- Input Handling
- Graphics and Animation
- Sound
- Physics
- ...

At this point our software engineering senses are tingling, something is dangerous here.

A better alternative in bigger projects is splitting the monolithic class and create different components and allow for their reuse later. Enter the Component pattern.

![Diagram of the Component Design Pattern](./images/patterns_containers/class_component.pdf){width=40%}

The client is connected to a list of Components that have the same interface (in the previous case, the `update()` method), so each Game Entity can become a "container of components" that define its behaviour.

For instance, instead of having all the functionalities listed above, our game entity could have the following components:

- Input Component
- Graphics Component
- Sound Component
- Physics Component

Which can be reused, extended and allow for further flexibility and follows more closely the DRY principle.

Here we can take a look at a sample implementation of the Component Design Pattern:

\code{patterns_containers/component}{Example Implementation Of the Component Pattern}

Resource Manager
-----------------

A useful container is the "resource manager", which can be used to store and manage textures, fonts, sounds and music easily and efficiently.

A resource manager is usually implemented via generic programming, which helps writing DRY code, and uses search-efficient containers like hash tables, since we can take care of loading and deletion during loading screens.

First of all, we need to know how we want to identify our resource; there are many possibilities:

- **An Enum:** this is usually implemented at a language-level as an "integer with a name", it's light but every time we add a new resource to our game, we will need to update the enum too;
- **The file path:** this is an approach used to make things "more transparent", but every time a resource changes place, we will need to update the code that refers to such resource too;
- **A mnemonic name:** this allows us to use a special string to get a certain resource (for instance "skeleton_spritesheet"), and every time our resource folder changes, we will just need to update our loading routines (similarly to the Enum solution).

Secondarily, we need to make sure that the container is **thread-safe** (see more about multithreading in the [multithreading section](#multithreading)), since we will probably need to implement a threaded loading screen (see how to do it [here](#loadingscreen)) to avoid our game locking up during resource loading.

{{placeholder}}

<!-- TODO: An associative container that contains pairs (ENUM, ITEM) used to store and retrieve
objects for the game -->

Animator
---------

This can be a really useful component to encapsulate everything that concerns animation into a simple and reusable package.

The animation component will just be updated (like the other components) and it will automatically update the frame of animation according to an internal timer, usually by updating the coordinates of the rectangle that defines which piece of a sprite sheet is drawn.

{{placeholder}}

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

{{placeholder}}

<!-- TODO: A simple finite state machine that allows to change states, useful for menus and stuff -->

Menu Stack
-----------

Although menus can be represented via a finite state machine, the structure of an User Interface (UI) is better suited for another data model: the stack (or rather, something similar to a stack).

A stack allows us to code some other functions in an easier way, for instance we can code the "previous menu" function by just popping the current menu out of the stack; when we access a new menu, we just push it into the menu stack and the menu stack will take care of the rest.

Unlike the stacks we are used to, the menu stack can also be accessed like a queue (first in - first out) so you can draw menus and dialogs on top of each other, while the last UI element (on top of the stack) keeps the control of the input-update-draw cycle.

In the menu stack we also have some functionalities that may not be included in a standard stack, like a "clear" function, which allows us to completely clean the stack: this can prove useful when we are accessing the main game, since we may not want to render the menu "below" the main game, wasting precious CPU cycles.

{{placeholder}}

<!-- TODO: Make a structure with a menu stack with "UI Elements" that can refer to the stack itself (for the "back" action for instance), add diagram and code -->

Particle Systems
-----------------

<!-- TODO: Talk about particle systems, particles and particle emitters -->

{{placeholder}}

### Particles

{{placeholder}}

<!-- TODO: What is a particle, what does it do, etc... -->

### Generators

{{placeholder}}

<!-- TODO: What is a particle generator -->

### Emitters

{{placeholder}}

<!-- TODO: What is a particle emitter, how does it differ from a generator -->

### Updaters

{{placeholder}}

<!-- TODO: What is a particle updater, how it works, etc... -->

### Force Application

{{placeholder}}

<!-- TODO: Talk about applying forces to particles -->

Timers
------

{{placeholder}}

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

\code{patterns_containers/easeInOut}{Ease-in-out}

Obviously these functions have an issue: they don't clamp the value between 0 and 1, that will have to be done in the movement function or by adding a check, or using some math, for instance using `min(calculated_value, 1)`.

\code{patterns_containers/clamping}{Clamping values}

In that case, calling the clamping function with values `0` and `1` would solve the issue.

A think that many people tend to forget, but that is really important is that you can tween any property of any entity or object in your game, for instance:

- The position of a UI item;
- The width of a health bar;
- The rotation of an on-screen map or compass;
- The colour of the sky while doing a day-to-night transition.

In short: any numeric value that can transition "smoothly" between two values in a certain amount of time can be tweened.

Chaining
--------

{{placeholder}}

<!-- TODO: An abstraction that allows to chain operations one after another, usually said operations are timed or related to tweening -->
