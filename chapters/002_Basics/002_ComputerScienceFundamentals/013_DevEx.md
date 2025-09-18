Developer Experience (DX/DevEx)
-------------------------------

Much like "user experience", developer experience (DevEx for short) is something that should be kept in high regard.

Developer experience encompasses everything that concerns the interaction between a developer and their work environment, which could be their tools, the work environment including culture, and anything that could affect their productivity and satisfaction.

This may also include the code itself: good code breeds other good code, while bad code will encourage bad practices (similar to the "Broken Window Principle").

What to do when coding
----------------------

Here are some small universal tips that you can use when coding that will help you, no matter the programming language you're in. These will usually avoid a lot of headaches by covering cases that may get overlooked or by giving guidance on how to solve some problems.

### Safe default initialization

Every time you make use of a variable, array or any data structure, you should always have it initialized to a known value.

Never leave variables in an uninitialized state: not all programming languages do what you might think, some may initialize an integer to a default value (zero), while others may leave it to whatever was in memory already (which would result in essentially a random value). If you make use of such variables as indexes inside arrays, you may find yourself in front of surprising behaviours.

The same goes for arrays, always make sure that all the array "cells" contain a known, valid and usable value from the very beginning, depending on your objective.

#### "Smart" default initialization

As a corollary of the "safe default initialization" theme, we can make use of a trick: if a certain array is known to contain mostly a certain value, provided some exceptions, it may be smarter to initialize it in a for loop and then change the exceptions manually.

Imagine an array containing "character expressions" that usually has a "neutral expression", but in a few occasions it may contain an angry or happy expression.

You may be tempted to initialize the array this way:

```{src="computer_science/naive_array_init" caption="A naive array initialization"}
```

This way gives a bad developer experience: every time you encounter feature creep (and trust me, you will), you will have to update the size of the array, create a new line with the initialization value (provided you don't forget and end up with a crash due to out-of-bounds indexing) and find yourself with a ton of lines of code doing the same thing.

A smarter way to do this could be the following:

```{src="computer_science/smart_array_init" caption="A smarter array initialization"}
```

Using a for loop to take care of the default values and then taking care of the exceptions is:

- **Shorter:** Doesn't matter how many items the array has, the default initialization code will stay the same;
- **Safer:** If you extend the array but forget to change a value, a character might have the wrong expression, but the game won't crash;
- **More flexible:** Since you won't have to worry about as much code as in the "naive version", it will be easier to expand upon your game.

::: tip :::
Use Enums for an even more readable code, since in most languages Enums map to integers, you can loop through an array of integers and then use the "named integers" offered by Enums to make the code even better.
:::::::::::

### Abstraction

{{placeholder}}
<!-- TODO: Use functions, classes and abstraction facilities -->


What to avoid when coding
-------------------------

Here are some examples of what are usually considered "universally bad practices" when coding, no matter the language you're using. There are usually referred as "code smells" and should be avoided as much as possible (but nothing is imperative in programming).

### Magic Numbers

Magic numbers (or more generally "magic constants") are numbers that can be found in your source code that have special meanings but such meaning cannot usually be quickly inferred without having to refer to somewhere else (either in the code or in the documentation).

Let's think of a simple piece of code:

```{src="computer_science/magic_numbers_1" caption="A piece of code featuring some magic numbers"}
```

It is difficult to understand what each value passed to the function means, so we would need to refer to the documentation or where such function was coded to be sure of what each constant means.

And now here's the same piece of code, but the "magic numbers" have been externalized into constants. This usually doesn't affect performance (most compilers replace constants with their inline values) and it is much more readable.

```{src="computer_science/magic_numbers_2" caption="A piece of code after magic numbers are removed"}
```

Avoiding magic numbers helps a lot with Developer Experience, especially when the "meaning" of such numbers is declared in a place far away from where they are used.

This approach also has some additional advantages, besides readability:

- **Ease of maintenance:** If one day we want to up the resolution of our game to 800x600, we just need to change two numbers, instead of running around the whole program, replacing 640 with 800 and 480 with 600, risking to replace some value that is not the screen size;
- **Resilience:** Making a typo on a number is easy: you can type 460 instead of 640 and you wouldn't be the wiser until a bug pops out. Making typos on words that make sense is harder, and your IDE will warn you about uninitialized variables/constants;
- **Grouping:** You can group all "constants" at the top of the file (or in a separate file altogether) and facilitate change and review.

### Code monoliths

Another thing that should usually be avoided is having long files. This usually is signal that whatever is in such file is taking care of too many things: you certainly can program an entire small game in a single file, but good luck with its maintenance.

Same goes for long classes, functions and pieces of code in general. These are usually called "code monoliths".

If classes are very long (as in "thousands of lines") maybe they're taking care of too many responsibilities, and need to be broken down into simpler pieces and then composed back together.

If functions are long maybe they're too complex, or they're doing more than one task (so they're like a small program by themselves), and may be useful to break them down in smaller functions that call each other, or are called by a function that works as "orchestrator".

Try to code your software with reuse in mind, if you find a pattern of very similar but slightly different pieces of code, maybe it's time to abstract them into something more generic that can be fed some parameters to do the same thing.
