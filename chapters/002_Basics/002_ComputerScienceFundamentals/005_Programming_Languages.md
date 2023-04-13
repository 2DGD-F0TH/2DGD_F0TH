Programming Languages
---------------------

Programming languages are a programmer's way to talk to a computer (or a console): they are a way to make an electronic apparatus do something (without involving analogue electronics).

### Classifying programming languages

Programming languages can be distinguished by many traits, it is important to know such differences, even though you may have already chosen your programming language.


#### By how they build

The way that a programming language gets you from code to "working product" can heavily influence both the final product as well as the speed of development.

##### Compiled Languages

Compiled languages need to go through a building process before it is possible for the product to be run anywhere. This has some advantages, as well as some disadvantages.

![Example of a compiler output (G++)](./images/computer_science/gpp_example.png){width=50%}

Among the disadvantages we have that the final product is usually non-portable, that means it cannot be run anywhere besides the machine it was compiled for. This means that you will have to create separate builds for each console, as well as different builds for each operating system.

Another disadvantage can be development speed: before you can test anything your game needs to be rebuilt. Sometimes the rebuild process can be quick (thanks to some techniques that avoid building things that didn't change), sometimes it can be long.

A very strong advantage of compiled languages is speed. Being essentially compiled to machine code, compiled languages have an easier time squeezing every last drop of performance from the platform you're building for. In addition, some languages can use features to physically remove unused code from the build: this way release builds can be much faster than debug ones, because the debug code is physically removed.

Among compiled languages we can find C and C++, as well as Rust and Go.

##### Interpreted Languages

Interpreted languages, in their strictest sense, are at the other side of the spectrum: the program is not compiled ahead of time but instead the source code is fed into an interpreter, which executes each row of instructions, one after the other.

Most interpreted languages feature an interactive *[REPL~\[g\]~](#gl_repl)* (read-eval-print loop) which allows to test code in real time.

![Python's REPL Shell](./images/computer_science/python_repl.png){width=50%}

They have the disadvantage of being usually slower than compiled languages and it's not easy to create builds that physically remove unused (debug) code without having to modify the sources manually. Also each console or operating system will need to have the interpreter installed, which may be an issue.

The advantage is in development speed: you can edit the source code and immediately run the interpreter to see the result, without having to wait for a new build to complete. Another advantage is portability: you don't need to create a new build for every system you want to run your game in, as long as an interpreter is available your game will run.

An example of a purely interpreted language is BASIC.

##### Hybrid Approaches

In any project, the ability to code quickly is as important as the performance of the final product: there is a thin balance to strike between "having a product with good performance" and "having a product that is released when needed". If your product releases too late, it doesn't matter how performing it is, the market will have chosen another product. If your product releases early but it underperforms, it will be replaced by better products.

Thus some hybrid approaches have been invented: one of these is, for instance, bytecode-compiled languages.

Bytecode-compiled languages (sometimes called "Languages with intermediate representation") are something that is not quite compiled, but it's not precisely interpreted either: the code is converted into bytecode, which is then fed to the interpreter (or "virtual machine") to run.

Being a representation that is "closer to the hardware" than the original source code, there is a gain in performance, while keeping the flexibility of interpreted code.

:::: trivia ::::
Some programming languages, like Haskell and Vala use the C programming language as an intermediate language, since C was meant to be an abstraction of the assembly language.
::::::::::::::::

Other approaches include Just-In-Time compiling, which trades off some longer starting times (sometimes called "warm-up times") for better overall performance.

Among the bytecode-compiled languages we can find Java and Python, while Lua can be considered a Just-In-Time Compiled language (thanks to LuaJIT).

#### By Paradigm

A programming paradigm is how the programming language lets you program. There is not a single, definitive way to code, thus programming languages can be distinguished by their paradigm.

##### Imperative Languages

Imperative languages are probably the most spread in modern programming: they make use of "orders" (called "statements") to change the status of the program.

This paradigm makes use of variables, statements, subroutines to make the program look like a set of instructions, a recipe, to make the program do what it needs to do (an algorithm).

Imperative languages include C, COBOL, Basic and Fortran.

##### Functional Languages

Functional languages make programs work by applying and composing functions (in the mathematical sense). Functions can be bound to variables and chained together (composed) to reach the result.

Functional languages include Haskell, Common Lisp and Scheme.

##### Multi-paradigm Languages

Many programming languages tend to "meld together" many programming paradigms, allowing (for instance) for functional style programming in imperative languages.

This means that functions can be bound to variables and passed around as any other object, they can be composed to reach the result if the programmer decides to do so (for instance for readability).

Multi-paradigm languages include Python, Lua and Go.

#### By the way types are determined

Sometimes underrated, how types are evaluated can completely change the way you program your game. Not knowing precisely how your language of choice treats types can lead to hard-to-debug issues.

##### Static Typing

Statically typed languages have their types decided ahead of time (usually when the program is compiled) and usually they cannot be changed.

This means that you have to have full awareness of which types will be used while writing your game. Which can be difficult at times.

Statically-typed languages include C, C++ and C#, as well as Java.

##### Dynamic Typing

Dynamically typed languages have their types decided at runtime. This allows for simpler syntax, but at the cost of lower performance, due to the fact that types are determined and verified at runtime.

Dynamically-typed languages include JavaScript and Ruby.

##### Duck Typing

Duck typing is probably the most misunderstood typing system. It can be described by the following sentence:

> If it walks like a duck and it quacks like a duck, then it must be a duck.

This means that types are inferred by their behaviour (their capabilities), thus creating a series of `-like` objects that behave more or less the same. This means that types can make use of the iteration capabilities of the language as long as they implement some basic methods that allow iteration (like `nextElement()` and `length()`).

This means that we have "file-like" objects, which behave like files, are used like files, but not necessarily have a counterpart in mass storage (they could be in-memory files), or "iterables" (sometimes called "list-like") which behave like lists of items, but may actually be something else (for instance strings could be seen as a "list of letters").

In the end, in duck typing, interfaces are treated as some kind of "informal protocol" that tells the language how to use an object. The "protocol" doesn't even need to be implemented fully: if you have a "file-like" object that implements only the reading method, you can still use it in the same way you'd use a file, as long as you don't try to write to it.

Duck Typing is used in the Python programming language.

#### By the "strength" of typing

How types are treated after each variable is instantiated can be the source of a lot of headaches while coding, thus it is paramount to be aware of how strong your preferred language's typing system is.

##### Strong Typing

Strongly typed languages don't allow one type to be treated like it was another type without an explicit conversion (usually called "cast"). This prevents unforeseen automatic type conversions that may lead to bugs and faults being undetected at compile time or runtime.

Some examples of strongly typed languages are C++, C#, Python and Java.

##### Weak Typing

Weak typed languages allow one type to be treated like another without explicit conversion. This may make the syntax simpler, but may be source of unforeseen bugs.

For instance a string may be treated as it was a number, this means that in some languages (where the operator `+` means both "addition between numbers" and "joining strings together") you may find that a result is a sum of numbers instead of two strings joined together.

An example of a weakly typed language is JavaScript.

:::: trivia ::::
What about the good old C language? C has strong typing for the great majority of the time, unless we consider the `void*` generic pointer. This kind of pointer can be used in other pointer variables without an explicit cast.
::::::::::::::::

#### By memory management

Another way to classify programming languages is how you can (or have to) manage your memory.

##### Languages without Garbage Collection

Some programming languages allow you to play with your system's memory as you wish: they give you all the tools (pointers, references, ...) to manually allocate and free memory.

This comes with its advantages and drawbacks: higher performance is surely a big advantage. A huge disadvantage is the fact that memory management is completely manual: dangling pointers and unreachable memory are commonplace, because there is nothing to clear after you.

Non Garbage-collected languages include C and C++.

##### Garbage-collected Languages

Some other languages prefer taking away part of the control on memory to help avoiding the problems that non Garbage-collected languages bring: there is something that cleans after you, which is the Garbage Collector.

The big disadvantage of this approach is that the garbage collector needs reference counting, CPU cycles to run, which means that the whole program runs slower.

Garbage collected languages include Java and Python.

### Languages available for this book

Here is a quick rundown of how the languages used in the various editions of this book (excluding "pseudocode", which is not really a programming language) are classified.

- **C++**, a compiled programming language with strong static typing. It is multi-paradigm (although it was born as an imperative language) and has no garbage collector.
- **JavaScript**, an interpreted language (although some engines support Just-In-Time compiling), with weak dynamic typing that supports some duck typing principles. It is multi-paradigm and features a garbage collector.
- **Lua**, a bytecode-compiled (or Just-In-Time compiled) language, with strong dynamic typing that supports some duck typing principles. It is multi-paradigm and garbage-collected.
- **Python**, a bytecode-compiled language, with strong duck typing. It is multi-paradigm and garbage-collected.

<!-- TODO: Add more languages as more editions come out -->
