Creational Design Patterns
--------------------------

Creational patterns is a category of design patterns that deal with object creation mechanisms, that means creating objects in a way that best suits the single situation.

### Singleton Pattern

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

If multiple threads are involved in using a lazy-loading singleton, you may need to take care of preventing *[race conditions~\[g\]~](#gl_racecondition)* that could result in multiple instances of the singleton being created.

Many critics consider the singleton to be an "anti-pattern", mostly because it is really overused and adds a possibly unwanted "global state" (see it as a global variable, but in an object-oriented sense) into the application.

Before applying the singleton pattern, ask yourself the following questions:

- Do I really need to **ensure** that only one instance of this object is present in the whole program?
- Would the presence of more than one instance of this object be detrimental to the functionality of this program? How?
- Does the instance of this object **need** to be accessible from everywhere in the program?

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Singleton

**When to Use it**    In all situations that strictly require one instance of an object, accessible globally.

**Advantages**        Allows the class to control its own instantiation, allows for easier access to the sole instance of a class.

**Disadvantages**     Introduces some restrictions that may be unnecessary, introduces a global state into the application.

------------------------------------------------------------------------------------------------

Table: Summary table for the Singleton Pattern

### Dependency Injection

{{placeholder}}

<!-- TODO: Dependency injection design pattern -->

### Prototype

Sometimes, in our game, we need to decide which objects to create at runtime, as well as instantiate dynamically loaded classes. In these cases the prototype pattern comes to the rescue: we define a "prototype" that allows to create classes by cloning itself.

There is the UML diagram for the pattern:

![Diagram of the Prototype Pattern](./images/design_patterns/class_prototype.svg){width=50%}

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Prototype

**When to Use it**    When you need to either decide the objects to create at runtime or instantiate dynamically loaded classes.

**Advantages**        Decoupling, added flexibility.

**Disadvantages**     May become overused, depending on the situation can be difficult to implement.

------------------------------------------------------------------------------------------------

Table: Summary table for the Prototype design pattern

{{placeholder}}

<!-- TODO: add code for the prototype pattern -->
