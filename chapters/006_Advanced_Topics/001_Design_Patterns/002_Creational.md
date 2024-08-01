Creational Design Patterns
--------------------------

Creational patterns is a category of design patterns that deal with object creation mechanisms, that means creating objects in a way that best suits the single situation.

### Singleton Pattern

Sometimes it can be necessary to ensure that there is one and only instance of a certain object across the whole program, this is where the *singleton* design pattern comes into play.

To make a singleton, it is necessary to hide (make private) the class constructor, so that the class itself cannot be instantiated via its constructor.

After that, we need a static method that allows to get the singleton's instance, the method needs to be static to make it callable without an instance of the singleton class.

The UML diagram for a singleton is really simple.

![The UML diagram for a singleton pattern](./images/design_patterns/singleton.svg){width=20%}

Some singleton implementations may instantiate themselves immediately, which is not always be necessary, in that case a good idea could be implementing the so-called "lazy loading", where the instantiation happens the first time you ask the object for its own instance.

```{src='design_patterns/singleton' caption='Example of a singleton pattern with lazy loading'}
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

Dependency Injection is a very simple concept that is really hard to explain. It is essentially used to avoid having classes build instances of services (which may be other classes or functions) inside of themselves and instead receiving such services from outside.

Let's make a concrete example. You have a class that takes care of everything concerning a file upload: from getting it from the internet, to logging to saving it to the hard disk.

A first implementation would look something like the following:

![A naive implementation of a local file upload system](./images/design_patterns/file_upload_naive.svg){width=50%}

What would happen if, instead of a hard disk you need to transfer the files to an external service like S3, or maybe it just needs to be saved into memory for further processing? You would probably need to duplicate the class to allow for these new "services".

![A naive implementation of a file upload system on S3](./images/design_patterns/file_upload_naive_s3.svg){width=50%}

A better solution for reuse would be having the "file saving service" separated from the entire "file upload" class, and instead having this service "injected" onto the "file upload" class. This can happen via setter functions, via constructors, builders, factories or even interface injection (where it's the dependency interface that provides a method to inject the dependency).

![Using Interfaces and DI to build a flexible file upload](./images/design_patterns/file_upload_di.svg){width=75%}

Since now the "file upload" class doesn't depend on how or where the file is saved, you can also substitute such "file saving service" with a mock during tests.

![Possible class structure for a DI file upload](./images/design_patterns/dependency_injection.svg){width=75%}

Dependency injection can be divided in two main categories:

- **Setter injection:** In this case, an object has specific `set()` methods that allow you to set the dependency after the object has been constructed. This allows also for changing the dependency on the fly (useful to change the effect of a weapon, for instance);
- **Constructor injection:** In this case, the injection happens in the class's constructor. The injected functionality is decided before the object that needs the dependency is constructed.

{{placeholder}}

<!-- TODO: Code for dependency injection design pattern -->

------------------    ------------------------------------------------------------------------------
**Pattern Name**      Dependency Injection

**When to Use it**    In all situations that require a degree of configurability or where the behaviour of the code needs to be changed without direct editing.

**Advantages**        Decreased coupling, reusability, maintainability, flexibility, less boilerplate code, allows for concurrent development of services, allows for easier unit testing.

**Disadvantages**     Behaviour and construction are separated, which may make tracing code harder. May hinder IDE automation if implemented using reflection or dynamic programming.

------------------------------------------------------------------------------------------------

Table: Summary table for the Dependency Injection Pattern

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
