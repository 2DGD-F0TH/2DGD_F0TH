Basic Data Structures
---------------------

{{placeholder}}

<!-- TODO: Small intro -->

### Statically-Allocated Arrays

{{placeholder}}

<!-- TODO: Talk about what are statically-allocated arrays (normal arrays) -->

### Structs

{{placeholder}}

<!-- TODO: Talk about structs/composite structures (can be "dicts" for Python) -->

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
