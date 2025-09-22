Basic Data Structures
---------------------

Before diving into the more complicated and customizable structures, we should familiarize ourselves with those basic data structures that are offered by essentially all programming languages in one way or another.

These will be the foundational blocks upon which we'll build more advanced structures.

### Statically-Allocated Arrays

Statically-Allocated arrays are a data structure that is equivalent to a list of variables of the same type, and such list is referred by the variable name.

![Example of a 4-element integer array](./images/computer_science/static_array.svg){width=30%}

At a lower level a certain amount of memory is allocated, depending on the type of items contained in the array (ints, strings, classes, ...) and how many items such array should contain at most. You can think of it as some machine code that allocates a defined number of "cells" that are contiguous in memory, dedicated to your array.

Since the program needs to allocate memory and the amount of memory allocated depends on how many items are contained in the array, such number of items is defined while coding and changing size is not really possible (there will be solutions in the [advanced data structures section](#advanced_data_structures)).

Given the structure of "adjacent cells", arrays are efficient at being iterated and directly indexed (so you can browse through all the items quickly, or find an item quickly if you know its position), but they are not as efficient in searching its contents (there will be solutions in the [advanced data structures section](#advanced_data_structures)).

### Structs

Structs are composite data types that are composed of other, simpler, data types.

Sometimes structs may be mixed up with classes, but the difference usually is that structs have data storage as their objective, while classes may have functions (called "methods") to work on themselves or the data they contain (usually called "fields").

Structs can be used to group together a bunch of variables that are related to each other, without making a class.

Structs is mainly a C/C++ term, the most similar types in other languages are:

- Dicts in Python;
- Tables in Lua (which is also the only complex data type available);
- Objects in JavaScript;

### Enums

Enums (or "Enumerated Types") are types of variable which can take a limited set of values, which are decided by the programmer while writing code.

For instance we can have a "VehicleFuel" Enum that can only take the following values:

- Gasoline
- Diesel
- LPG

Usually Enums are used as constants to make the code more readable and the programmer doesn't really care how these types are represented in memory (they could be just numbers or something more complex).

Let's take a very common usage for Enums: removing magic numbers:

```{src='computer_science/enums1' caption='A code block featuring some magic numbers'}
```

If those comments weren't there, the code would be much harder to understand, and every time we need to work with "fuel types", we need to remember which "magic number" is which fuel type.

Now let's see the same code, using Enums.

```{src='computer_science/enums2' caption='A code block featuring enums for readability'}
```

A bit longer, but much easier to read, right? Notice the absence of comments to explain the code, the code explains itself.
