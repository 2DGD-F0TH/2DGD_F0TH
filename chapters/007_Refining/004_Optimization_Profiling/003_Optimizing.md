Optimizing your game
--------------------

After accurate profiling, you need to intervene and try to get more out of your code. In this section we'll talk about some guidelines and tips on how to optimize your game.

### Working with references vs. returning values

Depending on the programming language you're using, and the amount of internal optimization its compiler/interpreter has, you may have the possibility to choose between two main ways of working, when it comes to functions:

- Returning a value from a function;
- Passing a reference to variables into the function and use that reference in your function (for instance in C++).

"Value Copying" can be a real resource hog when your functions work with heavy data. Every time you return a value, instead of working on a reference, you are creating a new copy of the data you're working on, that will be later assigned.

This can happen also when passing parameters to a function (in this case you say the "parameter is passed by value"): a new copy of the parameter is created locally to the function, using up memory. "Value Copying" can help when you don't want to modify the data outside your function, but is a waste when instead you **want** to modify such values.

Using things like "references", "constant references" and "pointers" can be really precious in making your game leaner memory-wise, as well as saving you all the CPU cycles wasted in memory copying.

### Optimizing Drawing

This heavily depends on the type of framework and engine you are using, but a good rule of thumb is using the lowest amount of calls to the draw routines as possible: drawing something entails a great amount of context switching and algorithms, so you should do it only when necessary.

If your engine/framework supports it, you should use sprite atlases/batches, as well as other interesting structures like Vertex Arrays (used in SFML), which can draw many elements on the screen with only one draw call.

Another way to optimize your drawing routine is avoiding to change textures often: changing textures can result in a lot of context changes (like copying the new texture from the RAM to the GPU memory), so you should use only one oversized texture (in the form of a [Sprite Sheet](#SpriteSheets)) and draw only a part of it, changing the coordinates of the rectangle that gets drawn. This way you'll save the PC a lot of work.

#### Off-screen objects

Make sure that your engine doesn't try to draw objects on off-screen area (maybe on virtual surfaces): drawing is an expensive operation and we should do it on the smallest possible set of objects, which is the visible screen area (the viewport).

Drawing objects doesn't change their internal state, so you can keep updating the objects and then draw them only when they fall (even just partially) inside the display's viewport.

![Not putting off-screen objects in the drawing queue can be a good optimization](./images/profiling_optimization/off_screen_optimization.svg){width=40%}

Some engines may already take care of this optimization, but some lower-level libraries may also leave that optimization to you. A good way to test is drawing thousands (or even millions, with the help of a `for` cycle) of sprites off-screen without any update code (maybe in a specific project) and see if the engine slows down considerably the more entities are added.

### Reduce the calls to the Engine Routines

Some engines have routines that introduce sanity checks, logic optimizations and more, and calling such routines more than necessary can burden your game's performance, even worse when you're calling them per-frame.

If you want to move a character diagonally both up and right, don't do this:

```{src='optimization/double_move_call' caption='Double Engine Movement Call'}
```

As all the sanity checks in the `Move` function will be executed twice per frame (since we're in the "Update" function). Instead you should get the resulting movement vector first, and then use the `Move` function only once:

```{src='optimization/single_move_call' caption='Single Engine Movement Call'}
```

This way instead we're doing sanity checks and related operations only once, moving the character in its final position without wasting resources.

### Entity Cleanup and Memory leaks {#cleanup_leaks}

One of the biggest scourges in software development (and an even bigger one in game development) are memory leaks: the program allocates memory but doesn't release it properly.

Memory management (as well as any kind of "resource management") can be summarized in 4 phases:

- Acquisition;
- Initialization;
- Usage;
- Release.

This is especially annoying when languages that don't have automatic garbage collection (like C++) are involved, but it can affect any programming language. Memory management is hard, and we should always release any resource that we acquire as soon as we're done using it, but that's not always easy: for instance when loading and unloading levels is involved.

As mentioned before, this problem affects all languages, since some resources may be acquired by some "active code" that is actually never running, thus preventing the garbage collector from working as it should.

Besides "being careful" with your resource management, you can check for memory leaks by using specific tools.

### Using analyzers to detect Memory Leaks {#mem_analyzers}

When developing a game, there are a lot of tools that allow you to inspect your game and find possible memory leaks. Some are "static scanners" while other (usually called "dynamic testing tools") require the game to be running.

#### Static Scanners

These tools analyze the code without running it, checking the style and common bugs that can be inserted by mistake. An example of these static tools are "linters" (or linting tools).

Most of these tools are included in IDEs but some (like LLVM's scan-build) are standalone.

![An example screen from LLVM's scan-build](./images/profiling_optimization/scan_build.png){width=50%}

#### Dynamic testing tools

Some tools require the game to be running, some general-purpose ones are used to find memory leaks (like Valgrind), while others have more specific purposes and are usually integrated into the engine.

![A screenshot from Valgrind, looks like we have a memory leak here](./images/profiling_optimization/valgrind.png){width=50%}

These more specific tools can track the FPS, memory as well as the calls done to each function, allowing you to track down what is bogging down your game.

![A screenshot from Godot's profiler](./images/profiling_optimization/godot_profiler.png){width=50%}

### Resource Pools {#res_pools}

Among the most performance-hungry operations in computers we find instantiation and destruction of objects: they involve context switches in the CPU, memory allocation/freeing and a lot of other things.

If you find yourself needing to instantiate and destroy a lot of objects of the same type, you may want to consider a "resource pool" for such object.

A resource pool is a group of objects that is instantiated once, ready to use and kept in memory (eventually without updating the internal status of the "inactive objects") until needed.

![A resource pool instantiates objects and "keeps them" ready when needed](./images/profiling_optimization/resource_pool_init.svg){width=40%}

When you need one of the objects, instead of instantiating it (and thus allocating memory, changing CPU context, etc...) you just "pull" an item from the pool and change its internal state as needed (since the memory is already instantiated). This allows you to "move" the cost of instantiating the object to some place in your code where some delays are expected (for instance the loading screens, after you loaded your resources).

![Pulling an object from a resource pool](./images/profiling_optimization/resource_pool_pull.svg){width=40%}

When you're done, instead of destroying the class (thus calling memory free methods and changing the CPU context again), you "return" the item to the resource pool, ready for another round. This allows you to "move" the cost of destroying the object somewhere where slowdowns are acceptable, for instance (again) loading screens.

![Returning an object from a resource pool](./images/profiling_optimization/resource_pool_return.svg){width=40%}

Particle systems are a prime example of resource pools: instead of continuously creating and destroying particles, you create all the particle object in advance to recycle and reuse during the game.

### Lookup Tables

Inside older games, where CPU cycles were at a premium, a widely used trick to gain performance were "lookup tables".

These tables would store the result values for certain expensive functions, given certain inputs, thus replacing the expensive operation with a lookup inside a certain data structure (which is usually really fast).

![How a lookup table works](./images/profiling_optimization/lookup_tables.svg)

This has a tradeoff: you're trading CPU time for Memory space, since the lookup tables are meant to stay into RAM.

In modern games instances of lookup tables are as rare as hens' teeth, but it's an interesting historical view over some older forms of optimization.

### Memoization {#memoization}

Memoization (sometimes known as "tabling") is an optimization technique that consists in saving the result of an expensive function, as well as the function's arguments for later calls: this way when the same arguments are passed to the function, you can return the stored value instead of performing the calculation again.

This is due to the fact that functions are deterministic, so if you have the same inputs you will always receive the same outputs: this allows to minimize expensive computations at the expense of memory.

Obviously this technique can't really be applied to functions that make use of pseudo-random numbers and connected functionalities, because memoization would completely void such randomization.

Memoization is usually implemented via decorators that check if the arguments passed are inside a defined data structure (usually a hash table): if there is a hit, the result is returned immediately, if not the original (expensive) function is run and its result is memorized in said structure.

A simple memoization system could work like the following UML diagram:

![How a simple memoization pattern works](./images/profiling_optimization/memoization_simple.svg){width=50%}

The biggest problem with this simple approach is that every different call to the function would be memorized, this would end up eating more and more memory, without any form of control.

The solution is deciding the "table size" of the results we want to keep: sometimes keeping the 10 most recently used calls is enough, sometimes we need more. Being able to control this will allow us to fine-tune the CPU vs. memory balance.

![How a more complex memoization pattern works](./images/profiling_optimization/memoization_limited.svg){width=50%}

Considering what we've seen so far, we can say that memoization should be used only on functions that are:

- Expensive
- Called often with the same arguments

:::: pitfall ::::
If we start using memoization techniques on all functions, we may end up with a software that occupies a lot of memory without any significant speedup. Moderation is key.
:::::::::::::::::

Here's how a memoization pattern could be implemented:

```{src='optimization/memoization' caption='An example of memoization'}
```

### Approximations

Many times when developing games we don't need to have a value that is precise to the 10th decimal digit, that's where approximation comes into hand.

A prime example of approximation was used in Quake III Arena, via the algorithm known as "Fast Inverse Square Root". Back in 1999 calculating the inverse square root of a number was an expensive calculation for the CPU, so the developers decided to create an algorithm that would calculate an approximation quickly.

This was done by playing around with the floating point low-level structure and using a "magic constant" (`0x5f3759df`) to create a good "first guess", after that a single iteration of the [Newton-Raphson Method](#newtonmethod) is applied to refine the guess.

This proved to be faster than directly calculating a normalized vector (which uses a square root and a division, expensive at the time) and also faster than using a lookup table. The algorithm proved to be slower (and less precise) than the dedicated SSE instruction in the newer x86 CPUs.

### Eager vs. Lazy Evaluation

Lazy objects are yet another possibility when it comes to optimization, with some drawbacks: you create an object but the calculations related to its state are performed when the object is first used, instead of when it is constructed.

This can be really useful when you have a great quantity of items that you are iterating through, one at a time, but don't need the whole collection at hand at once. When it comes to collections, lazy objects help saving memory at the cost of more CPU cycles while the game is running.

In some languages, this concept is abstracted in a language feature (like "generator expressions" in Python), while in others you'll have to work a little bit harder to get them.

Let's take an example, we have a custom object that contains a reference to a list of numbers: when we iterate through this object, we want it to return the numbers saved, halved.

::: note :::
What follows is just a didactic example, but should be simple enough to understand the difference between "eager" and "lazy" objects.
::::::::::::

#### Eager approach

The eager approach is to take the list of numbers, create a second list inside our object with the numbers halved: this will make sure that the values are always ready and readily available, but will consume more memory. Here's the example:

```{src='optimization/eager_mode' caption='An eager object'}
```

#### Lazy approach

If we know that we are working with millions of values, and we are going through them kind of rarely, saving all the halved values in RAM may not be a good idea. This is where lazy evaluation comes into play: instead of memorizing the value in RAM, we calculate it on-demand. Here's the example:

```{src='optimization/lazy_mode' caption='A lazy object'}
```

::: tip :::
Lazy objects are great when you're working with bigger-than-RAM lists: each single value is in memory when needed, instead of the whole list. This comes at a cost: when you need to iterate on such list multiple times, the cost or re-evaluating the result may become a problem.
:::::::::::

### Detach your updating from drawing

An interesting approach to squeeze a bit more performance from your game could be detaching your updates from the drawing routine. For instance, your game could be refreshing its screen at 60fps, but its internal state is updated only 20 times a second.

This will obviously introduce some complications, since you may need some interpolation to make things work smoothly. This approach will get rid of a lot of heavy work (in the case above, you will get rid of $\frac{2}{3}$ of the updates), freeing resources for new things.

This means that you can process AI less times (thus "check for player's presence" once every 50ms instead of once every 16), physics can be processed less too (we don't care if a block starts falling after 15 or 50ms, the time is still too short for us to notice).

Detaching your updates from the drawing routines usually entails a change of language too: when you're talking about drawing cycles, we talk about "frames", while when you're talking about updates, you should be talking about "game ticks".

{{placeholder}}
