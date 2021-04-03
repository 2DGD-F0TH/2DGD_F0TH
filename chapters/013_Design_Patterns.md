{{pagebreak}}

Design Patterns
================

:::::: {.epigraph author="Rich Hickey"}
Patterns mean "I have run out of language."
::::::

Design Patterns are essentially "pre-made solutions for known problems" and can help decoupling elements of your game, increasing maintainability.

Obviously design patterns are not a cure-all, they can introduce overhead and could lead to over-engineering: balance is key when it comes to creating a game (or any software in general).

Singleton Pattern
-----------------

Sometimes it can be necessary to ensure that there is one and only instance of a certain object across the whole program, this is where the *singleton* design pattern comes into play.

To make a singleton, it is necessary to hide (make private) the class constructor, so that the class itself cannot be instantiated via its constructor.

After that, we need a static method that allows to get the singleton's instance, the method needs to be static to make it callable without an instance of the singleton class.

The UML diagram for a singleton is really simple.

![The UML diagram for a singleton pattern](./images/design_patterns/singleton.svg){width=20%}

```{src='design_patterns/singleton' caption='Example of a singleton pattern'}
```

The previous singleton instantiates immediately, which may not always be necessary, in that case a good idea could be implementing the so-called "lazy loading", where the instantiation happens the first time you ask the object for its own instance.

```{src='design_patterns/singleton_lazyload' caption='Example of a singleton pattern with lazy loading'}
```

If multiple threads are involved in using a lazy-loading singleton, you may need to take care of preventing *race conditions*~[g]~ that could result in multiple instances of the singleton being created.

Many critics consider the singleton to be an "anti-pattern", mostly because it is really overused and adds a possibly unwanted "global state" (see it as a global variable, but in an object-oriented sense) into the application.

Before applying the singleton pattern, ask yourself the following questions:

- Do I really need to **ensure** that only one instance of this object is present in the whole program?
- Would the presence of more than one instance of this object be detrimental to the functionality of this program? How?
- Does the instance of this object **need** to be accessible from everywhere in the program?

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Singleton

**When to Use it**    In all situations that strictly require one instance of an object, accessible globally.

**Advantages**       Allows the class to control its own instantiation, allows for easier access to the sole instance of a class.

**Disadvantages**     Introduces some restrictions that may be unnecessary, introduces a global state into the application.

------------------------------------------------------------------------------------------------

Table: Summary table for the Singleton Pattern

Command Pattern
---------------

It may be necessary, during our software development, to abstract our functions into something that can be assigned and treated as an object.

Many programming languages now feature functions as "first class citizens", allowing to treat functions as objects: assigning functions to variables, calling functions, lambdas, inline functions, functors, function pointers...

The command pattern allows us to abstract a function (or any executable line of code) into its own object that can be handled as such, allowing us to package a request into its own object for later use.

This pattern can be useful to code GUIs, making actions in our games that can be undone, macros, replays and much more.

![UML diagram for the Command Pattern](./images/design_patterns/command.svg){width=30%}

```{src='design_patterns/command' caption='Example code for the Command Pattern'}
```

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Command

**When to Use it**    In all situations where you want to avoid coupling an invoker with a single request or when you want to configure an invoker to perform a request at runtime.

**Advantages**       Allows for encapsulation, less coupling, more flexibility and customization at runtime.

**Disadvantages**     Late binding and objects may introduce some overhead.

------------------------------------------------------------------------------------------------

Table: Summary table for the Command Pattern

Flyweight
---------

Sometimes it may be necessary to keep track of a large number of very similar objects.

Imagine a lot of sprites of trees that have the same texture and size, but have different positions: it could prove to be really resource-heavy to keep all the sprite objects in memory, each one with its own copy of the texture and size. This could prove to be performance-heavy too, since all those textures will have to be moved to the GPU.

Here comes the Flyweight pattern: we try to share as much of the so-called "intrinsic state" of the objects between the object that contain the so-called "extrinsic state".

![UML Diagram of the Flyweight pattern](./images/design_patterns/flyweight.svg){width=60%}

Below is an example code for the flyweight pattern.

```{src='design_patterns/flyweight' caption='Code for a flyweight pattern'}
```

::: trivia :::
This is just speculation, but SFML's graphics system may make use of the Flyweight pattern: you need to load the image into a "Texture" first (which does all the low-level lifting) and then you can instance an "Image" class which is more high-level. Many images can refer to the same texture (which may be a Sprite Sheet).
::::::::::::::

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Flyweight

**When to Use it**    When you need to support a large number of similar objects efficiently, when you need to avoid creating a large number of objects.

**Advantages**       Saves memory when a large number of similar objects is involved, avoids some of the overhead given by the creation of many objects.

**Disadvantages**     The intrinsic state must be "context independent", so it cannot change (or all the flyweights that refer to that state will change too). Flyweight instantiation requires particular attention in multithreaded environments, due to the shared memory.

------------------------------------------------------------------------------------------------

Table: Summary table for the Flyweight Pattern

Observer Pattern {#ObserverPattern}
-----------------------------------

The observer pattern is used to implement custom event handling systems, where an object automatically reacts to events generated by another object.

There are 2 main objects used in an observer pattern:

- **The Subject**: sometimes called "Observed Object"
- **The observer**: sometimes called "Dependent Object"

The subject is the creator of a "stream of events" that is consumed by the observer objects.

The subject implements in its structure a list of observers that will be notified when a change occurs, as well as methods to register (add) a new observer as well as to unregister (remove) an existing observer, while the observers will implement a method that will be called by the subject, so that the observers can be notified of such change.

Here we can see an UML diagram of the observer pattern:

![The UML diagram of the observer pattern](./images/design_patterns/observer.svg){width=60%}

Here we can see the Observer abstract class (it can be an interface), a concrete subject and two Concrete Observers that implement what required by the Observer.

Here we can see an implementation of the observer pattern:

```{src='design_patterns/observer' caption='Code for an observer pattern'}
```

If needed, you can pass information between the subject and the observers just by calling each `update()` method with the necessary arguments.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Observer

**When to Use it**    Event Handling systems, making objects react to other objects' actions

**Advantages**       Decoupling, added flexibility, more performant than if statements for conditions that happen rarely.

**Disadvantages**     Can be a bit hard to set up, makes the architecture more complex, if un-registration is not done well there could be serious memory leaks (even in garbage-collected languages).

------------------------------------------------------------------------------------------------

Table: Summary table for the Observer Pattern

Strategy
--------

In some situations it may be necessary to select a single algorithm to use, from a family of algorithms, and that decision must happen at runtime.

In this case, the *strategy pattern* (also knowns as the "policy pattern"), allows the code to receive runtime instructions over what algorithm to execute. This allows for the algorithm to vary independently from the client that makes use of such algorithm.

![The UML diagram of the strategy pattern](./images/design_patterns/strategy.svg){width=60%}

```{src='design_patterns/strategy' caption='Code for a strategy pattern'}
```

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Strategy

**When to Use it**    Every time you need to decide which algorithm to execute at runtime.

**Advantages**       Decoupling, added flexibility.

**Disadvantages**     Can cause proliferation of similarly-looking concrete strategies, late binding on functions and the object oriented nature of the pattern could create some overhead.

------------------------------------------------------------------------------------------------

Table: Summary table for the Strategy Pattern

Chain of Responsibility
-----------------------

Sometimes we have the necessity of handling conditionals that are themselves connected to runtime conditions. This is where the *chain of responsibility pattern* comes into play, being essentially an object-oriented version of an `if ... else if ... else` statement.

![UML Diagram of the Chain of Responsibility Pattern](./images/design_patterns/chain_of_responsibility.svg){width=60%}

As can be seen from the diagram, the sender is not directly connected to the receiver, but instead it's connected to a "Handler" interface, making them independent.

As with a chain of responsibility in a company relays a task to "higher ups" if the task cannot be handled, the chain of responsibility pattern involves each received reviewing the request and if possible, process it, if not possible, relay it to the next receiver in the chain.

```{src='design_patterns/chain_of_responsibility' caption='Code for a chain of responsibility pattern'}
```

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Chain of Responsibility

**When to Use it**    When you need to implement flexible if...else if...else statements that change on runtime. When you want to decouple a sender from a receiver.

**Advantages**       Decoupling, added flexibility.

**Disadvantages**     Some overhead is added by the objects and late binding, could lead to proliferation of similar-looking handlers/receivers.

------------------------------------------------------------------------------------------------

Table: Summary table for the Chain of Responsibility Pattern

Component/Composite Pattern
---------------------------

When building any game entity, we find that the complexity of the game entity itself literally explodes: a monolithic class can include loads of different operations that should stay separate, such as:

- Input Handling
- Graphics and Animation
- Sound
- Physics
- ...

At this point our software engineering senses are tingling, something is dangerous here.

A better alternative in bigger projects is splitting the monolithic class and create different components and allow for their reuse later. Enter the Component pattern.

![Diagram of the Component Design Pattern](./images/design_patterns/class_component.svg){width=40%}

The client is connected to a list of Components that have the same interface (in the previous case, the `update()` method), so each Game Entity can become a "container of components" that define its behaviour.

For instance, instead of having all the functionalities listed above, our game entity could have the following components:

- Input Component
- Graphics Component
- Sound Component
- Physics Component

Which can be reused, extended and allow for further flexibility and follows more closely the DRY principle.

Here we can take a look at a sample implementation of the Component Design Pattern:

```{src='design_patterns/component' caption='Example Implementation Of the Component Pattern'}
```

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Component/Composite

**When to Use it**    When you need to deal with a part-whole hierarchy where each component needs to be treated equally.

**Advantages**        Decoupling, added flexibility.

**Disadvantages**     On bigger systems, the management may become really complex.

------------------------------------------------------------------------------------------------

Dependency Injection
--------------------

{{placeholder}}
<!-- TODO -->

Decorator
---------

There are some cases where we need to add or remove behaviours from a class at runtime, dynamically. The decorator pattern gives a flexible alternative to subclassing and addresses this need.

![Diagram of the Decorator Pattern](./images/design_patterns/class_decorator.svg){width=40%}

As you can see the decorator makes heavy use of abstract classes and interfaces, which most programming languages implement without any issue.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Decorator

**When to Use it**    When you need to add or remove functionalities from a class dynamically.

**Advantages**        Decoupling, added flexibility.

**Disadvantages**     Usage of interfaces or abstract classes can seem a bit daunting at the beginning, it may cause an explosion in number of classes.

------------------------------------------------------------------------------------------------

{{placeholder}}
<!-- TODO: Code for the decorator pattern -->

Visitor
-------

{{placeholder}}
<!-- TODO -->

Adapter
-------

{{placeholder}}
<!-- TODO -->

### Class Adapter

{{placeholder}}
<!-- TODO -->

### Object Adapter

{{placeholder}}
<!-- TODO -->

Prototype
---------

{{placeholder}}
<!-- TODO -->

Facade
------

There are times where you have a very complex library, with a very complex interface, that is extremely complex to interact with. The Facade pattern hides such complexity behind a simple-to-use interface that works by delegation.

![Diagram of the Facade Pattern](./images/design_patterns/class_facade.svg){width=40%}

This pattern should be used with extreme care and only when necessary, since adding "levels of indirection" will make the code more complex and harder to maintain.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Facade

**When to Use it**    When you need to present a simple interface for a complex system or you want to reduce the dependencies on a subsystem.

**Advantages**        Decoupling, added readability.

**Disadvantages**     May become overused, delegating adds a bit of overhead, sometimes it may be wrongly used where either an adapter or a decorator is needed.

------------------------------------------------------------------------------------------------

<!-- TODO: Code for the facade pattern -->
