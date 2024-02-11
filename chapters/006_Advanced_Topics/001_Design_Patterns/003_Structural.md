Structural Design Patterns
--------------------------

Structural patterns is a category of design patterns that deals with the relationships between entities, with the objective of simplifying the realization of such relationships.

### Flyweight {#flyweight}

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

**Advantages**        Saves memory when a large number of similar objects is involved, avoids some of the overhead given by the creation of many objects.

**Disadvantages**     The intrinsic state must be "context independent", so it cannot change (or all the flyweights that refer to that state will change too). Flyweight instantiation requires particular attention in multi-threaded environments, due to the shared memory.

------------------------------------------------------------------------------------------------

Table: Summary table for the Flyweight Pattern

### Component/Composite Pattern

When building any game entity, we find that the complexity of the game entity itself literally explodes: a monolithic class can include loads of different operations that should stay separate, such as:

- Input Handling
- Graphics and Animation
- Sound
- Physics
- ...

At this point our software engineering senses are tingling, something is dangerous here.

A better alternative in bigger projects is splitting the monolithic class and create different components and allow for their reuse later. Enter the Component pattern.

![Diagram of the Component Design Pattern](./images/design_patterns/class_component.svg){width=50%}

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

:::: tip ::::
You can also think about components as "capabilities": objects can be "movable", so they have an input or physics component, they can be "drawable" so they have a graphics component, etc...
:::::::::::::

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Component/Composite

**When to Use it**    When you need to deal with a part-whole hierarchy where each component needs to be treated equally.

**Advantages**        Decoupling, added flexibility.

**Disadvantages**     On bigger systems, the management may become really complex.

------------------------------------------------------------------------------------------------

Table: Summary table for the Component/Composite design pattern

### Decorator

There are some cases where we need to add or remove behaviours from a class at runtime, dynamically. The decorator pattern gives a flexible alternative to subclassing and addresses this need.

![Diagram of the Decorator Pattern](./images/design_patterns/class_decorator.svg){width=40%}

As you can see the decorator makes heavy use of abstract classes and interfaces, which most programming languages implement without any issue.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Decorator

**When to Use it**    When you need to add or remove functionalities from a class dynamically. Very useful for applying [memoization techniques](#memoization).

**Advantages**        Decoupling, added flexibility.

**Disadvantages**     Usage of interfaces or abstract classes can seem a bit daunting at the beginning, it may cause an explosion in number of classes.

------------------------------------------------------------------------------------------------

Table: Summary table for the Decorator design pattern

{{placeholder}}

<!-- TODO: Code for the decorator pattern -->

### Adapter

Let's face it, not everything is straight out compatible with everything else. It happens with power plugs, why wouldn't it happen in a world as varied as software development?

Sometimes we need an adapter, and that's exactly what this design pattern is: provide a layer of compatibility between two incompatible interfaces.

The adapter design pattern can be implemented in two ways, but first let's check the summary table.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Adapter

**When to Use it**    When you need to provide a layer of compatibility between two incompatible interfaces or you need to provide an "alternative interface" to a class.

**Advantages**        Decoupling, added flexibility, better compatibility, code reuse.

**Disadvantages**     Needing many adapters may mean there is a deeper structural problem with your program.

------------------------------------------------------------------------------------------------

Table: Summary table for the Adapter design pattern

#### Object Adapter

The "object adapter" is the version where the adaptor delegates the task to the adaptee at runtime. To do so, it has an instance of the adaptee class as its class field.

![Diagram of the Object Adapter Pattern](./images/design_patterns/class_object_adapter.svg){width=50%}

{{placeholder}}

<!-- TODO: Code for object adapter -->

#### Class Adapter

The "class adapter" version instead inherits the adaptee class at compile-time. Since the adaptor inherits from the adaptee class, the adaptee's methods can be called directly, without needing to refer to a class field.

![Diagram of the Class Adapter Pattern](./images/design_patterns/class_class_adapter.svg){width=50%}

{{placeholder}}

<!-- TODO: Code for class adapter -->

### Facade

There are times where you have a very complex library, with a very complex interface, that is extremely complex to interact with. The Facade pattern hides such complexity behind a simple-to-use interface that works by delegation.

![Diagram of the Facade Pattern](./images/design_patterns/class_facade.svg){width=50%}

This pattern should be used with extreme care and only when necessary, since adding "levels of indirection" will make the code more complex and harder to maintain.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Facade

**When to Use it**    When you need to present a simple interface for a complex system or you want to reduce the dependencies on a subsystem.

**Advantages**        Decoupling, added readability.

**Disadvantages**     May become overused, delegating adds a bit of overhead, sometimes it may be wrongly used where either an adapter or a decorator is needed. May become a [single point of failure~\[g\]~](#gl_spof).

------------------------------------------------------------------------------------------------

Table: Summary table for the Facade design pattern

Here's an example of a possible facade pattern:

```{src='design_patterns/facade' caption='Example Implementation Of the Facade Pattern'}
```

### Proxy

Sometimes, the access to a certain object may be problematic or not directly possible, for instance when:

- The object is available only remotely;
- The object needs a form of access control;
- The object is too heavy to complex to be available in memory at all times.

In these cases, the proxy pattern is the pattern that may solve your issues: in its most general form, a proxy is an interface to another object. Such interface may have additional operations attached to it (for instance for access control), or it could represent a "virtual object", where the real object is located somewhere else.

![Diagram of the Proxy Pattern](./images/design_patterns/class_proxy.svg){width=50%}

This can be very useful in implementing multiplayer systems: a well-designed system won't care if the input controlling a character come from the keyboard, a gamepad, another computer (while playing multiplayer) or a file (for replays, for instance). The proxy pattern will make it so that everything has the same interface and the "user object" (as in the object that uses the proxy) won't have to worry about what's on "the other side of the proxy".

```{src='design_patterns/proxy' caption='Example Implementation Of the Proxy Pattern'}
```

The proxy pattern is also very useful for lazy-loading: the Proxy object will pretend to be the original object, until the object is really needed (thus moving the instantiation weight to the "first use" of an object), then it will just "pass through" every command.

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Proxy

**When to Use it**    When you need to represent a remote object, a complex/heavy object that can't fit in memory or if you need to control access to an object. Useful for lazy loading, replays or remote play.

**Advantages**        Decoupling.

**Disadvantages**     When used for lazy-loading, "passing through" function calls will add function pointers to the stack, which may slow down performance.

------------------------------------------------------------------------------------------------
