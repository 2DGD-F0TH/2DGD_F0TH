Tips and tricks
---------------

### Be mindful of your "updates"

It is a common mistake among new game developers of putting the whole game logic inside the engine's `update()` method: this will eventually bog down the game and create inconsistencies when the framerate varies.

Input should be handled in your engine's event-based input system (very rarely you will need to check the keyboard status inside the `update()` method), also you should absolutely take advantage of your engine's facilities when it comes to managing how the game updates.

For instance, Unity offers 3 update functions:

- `FixedUpdate()`
- `Update()`
- `LateUpdate()`

`FixedUpdate()` is executed with the Physics engine, so here is where you should apply forces, torques and any other physics-related function. Being run with the physics engine, this function may be called zero, one or more times per frame.

`Update()` is your run of the mill update function, it is always executed once per frame, without fail. This is used for other kinds of updates, if you do physics operations here the results may be inconsistent (since it doesn't run in sync with the physics engine). You can still move objects that are not tied to physics.

`LateUpdate()` is a utility function that is run once per frame, after the `Update()` function. This is useful for all kinds of operations that would require the `update()` calculations to be completed.

### Use the right data structures for the job

Choosing the appropriate data structure for a task can have a lot more impact on performance that we may expect, and choosing the wrong one can have an even bigger impact.

So here are some small tips that work with the majority of programming languages.

Arrays are contiguous memory sections, thus indexing (finding an element at a certain position) is fast, as is scanning through the entire array itself. The limitation is that "pure arrays" have a well-defined size and cannot be resized: if you need a bigger array, you need to allocate memory for it and copy over the data. Inserting an item at the end of the array (if not full) is fast, but inserting an item at the head or in the middle of the array can be quite slow (since the items would need to be moved over).

Dynamic Arrays (sometimes called "Vectors") try to solve the "frozen size" of Arrays, while keeping the advantages. Pushing to the end of a dynamic array is usually fast (with the exception of the times the array is automatically resized to hold more items), but pushing items at the beginning (or in the middle) of the array is usually quite slow (because all items would need to be moved). Dynamic arrays tend to "overcommit memory", so they may be bigger than necessary (to save on the computationally heavy task of resizing and copying over items).

Linked lists are good if you need fast insertion anywhere, but they tend to lack in the iteration department: since the nodes are not contiguously packed in memory, iteration can be slow.

Hash tables are good if you need to memorize items in a "key-value" fashion and retrieve them very quickly, but they use more memory and may fall short in terms of performance if a bad hashing function is involved.

There is no "silver bullet" when it comes to data structures, but knowing the basics can make your code a lot better: there are more advanced data structures, like heaps, that are discussed in this book, check them out!

### Dirty Bit

Not all entities in your game need to have their state updated all the time. Continuously updating all entities' internal state can be really costly in terms of game performance.

A quick way to make your game lighter on resources (and thus more performing) can be putting a boolean check at the beginning of the update function, checking if the object really needs to have its internal state updated.

A possible example could be the following:

```{src='optimization/dirty_bit' caption='Example on how to optimize entities with a dirty bit'}
```

If your code is well-done, you won't have issues like animations freezing, because those will be separated from the "update routine", since the animator will chug along its frames when requested by the `draw` function.

### Far-Away entities (Dirty Rectangles)

Another way to optimize your game performance is not updating entities way off screen: this is also a technique used in the game Minecraft, where entities are frozen when you are far away from them, to save on resources.

A possible idea would be having an "updatable rectangle" (sometimes called "Dirty Rectangle"), bigger than the screen, and only the entities inside such rectangle will be updated.

This could create some issues when it comes to games that have their challenge deriving from entities updating in sync with each other, thus if we implement this "updatable rectangle" one or more entities would fall "out of sync", possibly making beating the level impossible.

In that case we may just put out an exception (where certain entities are updated no matter what) or divide our level into smaller "rooms" that are instead entirely updated all the time. Another way could be updating the internal state of the objects, but not drawing them at all, which would still lighten the workload.

:::: tip ::::
If you want to keep updating far away entities (for instance to avoid seeing all entities start updating as soon as they enter the screen), you can update entities every other frame, or just update one half of the entities on one frame and the other half on the next.
:::::::::::::

### Tweening is better than animating

Animators and animation frames are performance-hungry and should absolutely not be used in all those situations where you can instead use inbetweening techniques.

This means that frame-by-frame animations should not be used when taking care of moving UI parts: if you want to slide a piece of UI (take for instance a drop-down terminal from a "computer hacking" game) from the top, you can just tween its position and save a lot of memory.

Remember that Tweening doesn't apply only to positions, you can tween any property of a game object.

So a quick way you can optimize your game, is removing all the unnecessary animations and replace them with tweening, your game will surely benefit from that.

### Remove dead code

There are many definitions for "dead code", some use the "unreachable code" definition (for instance code placed after a "return statement") some use a more extensive definition.

I like to think of dead code as "wasted code", which is:

- Anything that happens to be written after a "return statement" in a function: return statements are used to give control of the program back to the caller of a function, so this code will never be executed;
- Unused variables: variables are allocated in memory, require calculations and CPU cycles, if not used that's just a waste;
- Unused code: complete functions that are never called are a waste of memory (because they may be loaded in RAM) and of disk space, making the executables bigger;
- Debug code: sometimes we need to write code to debug other code, this code may end up being part of a "release version" and weigh it down, this may also make the game more sensitive to cheating and hacking.

You should be careful when optimizing out dead code, even more when you are dealing with functions which result is not used: those functions may change some global state (or change stuff by usage of *[side effects~\[g\]~](#gl_sideeffect)*).
