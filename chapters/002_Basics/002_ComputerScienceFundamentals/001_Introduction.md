{{pagebreak}}

Some Computer Science Fundamentals
===================================

:::::: {.epigraph author="Edsger W. Dijkstra"}
The computing scientist's main challenge is not to get confused by the complexities of his own making.
::::::

In order to understand some of the language that is coming up, it is necessary to learn a bit of the computer science language and fundamentals.

This chapter will briefly explain some of the language and terms used, their meaning and how they contribute to your activity of developing games.

Algorithms
----------

{{placeholder}}

<!-- TODO: Intro to algorithms, what they are, just to bring people up to speed. More of a memo. -->

Number representations
----------------------

When you work with computers, it's impossible to avoid learning a bit of number representations. Computers work with a different logic than humans do: humans have complex minds and thoughts, while most of the time computers work in ones and zeroes. Most of what we see on a screen can be reduced to electrons going through a semiconductor in kind of an orderly fashion: changing from 0 volts (ground) to 5 volts.

### The most used representations

Here we will take a quick look at the most used representations. Some are more fundamental than others, but they are all useful in their own way.

Each representation will use a subscript to represent its representation. If no subscript is present it means the standard decimal representation is used.

#### Decimal

This is the standard decimal notation everyone is used to, we have 10 digits at our disposal:

$$
0\ 1\ 2\ 3\ 4\ 5\ 6\ 7\ 8\ 9
$$

And we place them in certain positions (units, tens, hundreds, thousands, etc...) to represent a certain quantity. We will use this as a basis for all other representations.

So if you want to represent $9 + 1$ you will use the $1$ digit, followed by the $0$ digit to make $10$.

#### Binary

This is the most used representation in computer science, we have only two digits at our disposal: $0\ 1$.

Thus if you want to make $1_{bin}+1_{bin}$, you will have to use the $1$ digit, followed by the $0$ digit, thus making $10_{bin}$, which is the binary representation of 2.

Here are the first 10 numbers for comparison purposes:

| Decimal | Binary |
| :-----: | :----: |
| 0       | 0      |
| 1       | 1      |
| 2       | 10     |
| 3       | 11     |
| 4       | 100    |
| 5       | 101    |
| 6       | 110    |
| 7       | 111    |
| 8       | 1000   |
| 9       | 1001   |
| 10      | 1010   |

Table: Comparison between decimal and binary representations

Binary numbers can be used at low level to represent any kind of "binary condition" too: yes/no, true/false are usually mapped to 1 and 0 respectively. This will probe useful in some cases where we will use "binary numbers" to represent groups of "binary conditions" in a compact way, but that's an advanced thing we'll see later.

#### Octal

In the octal representation we have 8 digits at our disposal:

$$
0\ 1\ 2\ 3\ 4\ 5\ 6\ 7
$$

Thus the representation of the decimal number $8$ in the octal system is $10_{oct}$.

The octal number system doesn't find much use in computer science besides being a quicker way to represent binary numbers. The conversion is quite easy and will be explained in a bit.

Here's a quick comparison between decimal and octal representations:

| Decimal | Octal  |
| :-----: | :----: |
| 0       | 0      |
| 1       | 1      |
| 2       | 2      |
| 3       | 3      |
| 4       | 4      |
| 5       | 5      |
| 6       | 6      |
| 7       | 7      |
| 8       | 10     |
| 9       | 11     |
| 10      | 12     |

Table: Comparison between decimal and octal representations

#### Hexadecimal

Hexadecimal is definitely the second most used representation in computer science, due to how easy it is to represent 4 bytes in a very compact notation.

In the hexadecimal notation, we have sixteen digits at our disposal:

$$
0\ 1\ 2\ 3\ 4\ 5\ 6\ 7\ 8\ 9\ A\ B\ C\ D\ E\ F
$$

Here's a table of the first 20 numbers to clarify a bit how things work:

| Decimal | Hex    |
| :-----: | :----: |
| 0       | 0      |
| 1       | 1      |
| 2       | 2      |
| 3       | 3      |
| 4       | 4      |
| 5       | 5      |
| 6       | 6      |
| 7       | 7      |
| 8       | 8      |
| 9       | 9      |
| 10      | A      |
| 11      | B      |
| 12      | C      |
| 13      | D      |
| 14      | E      |
| 15      | F      |
| 16      | 10     |
| 17      | 11     |
| 18      | 12     |
| 19      | 13     |
| 20      | 14     |

Table: Comparison between decimal and hexadecimal representations

### Converting between decimal and binary

The algorithm to convert between decimal and binary is quite simple. It is an iterative algorithm that consists in integer dividing the number by 2, until the result of the division is 1. The modulo of such divisions will make up our binary number.

An example is worth a thousand words: let's convert the number 38 to binary.

First of all, we integer divide 38 by two: the result is 19, there is no remainder, so we'll use zero.

| Dividend | Remainder |
| -------: | :-------- |
| 38       | 0         |
| 19       |           |

Let's continue: we integer divide 19 by two: the result is 9, with 1 as remainder.

| Dividend | Remainder |
| -------: | :-------- |
| 38       | 0         |
| 19       | 1         |
| 9        |           |

We iterate some more, by integer dividing until we get 1 as a dividend, at that point we make the last division, which will have remainder 1:

| Dividend | Remainder |
| -------: | :-------- |
| 38       | 0         |
| 19       | 1         |
| 9        | 1         |
| 4        | 0         |
| 2        | 0         |
| 1        | 1         |

Now we just need to read our remainders from bottom to top. So the binary representation of $38_{dec}$ is $100110_{bin}$.

:::: note ::::
This is actually a much more generic algorithm: you can convert from decimal to octal and hexadecimal for instance, just by dividing by 8 and 16 respectively. You can convert 38 to octal and hexadecimal as an exercise: the results are $46_{oct}$ and $26_{hex}$.
::::::::::::::

#### Two's complement

So far we've seen how to convert positive integers from decimal to binary, but how do we represent negative integers?

That's where "two's complement" representation comes into play: there is a bunch of theory behind why it's called this way, and how it works, but what we need to know will be how to represent a negative number.

> To represent a negative binary number in two's complement you invert all the bits of such number, then add 1.

As usual, an example is worth a thousand words. We want to convert the number $-38$ into binary.

First of all we need to define what our range of numbers will be, so that we know how many bits we will use. This is done because this range will be equally split between positive and negative numbers. In this example I will choose a normal 8-bit representation, which can represent numbers spacing from -128 to 127.

The first step is to convert $38$ to binary, which as we saw is $100110_{bin}$. We will pad this binary number to 8 bits, obtaining $00100110_{bin}$ as a result.

Now we just need to invert the all the bits in the number, obtaining $11011001_{bin}$ as a result.

Last step is adding 1 to what we got in the previous step, thus the final result is $11011010_{bin}$.

:::: note ::::
The more perceptive of you may have noticed a problem: what if we tried to represent the number 128 with 8 bits?

We would obtain $1000000_{bin}$ which is actually the representation of -128 in two's complement. This is called an "integer overflow", so be careful when mixing unsigned and signed integers.
::::::::::::::


#### Floating point

{{placeholder}}

<!-- TODO: Talk briefly about exponent+fraction representation and the lack of precision it may bring -->

### Converting between binary and octal

As mentioned before, octal can be used as a "shorthand way" to represent binary. The conversion is pretty simple.

> To convert from binary to octal, take the binary digits in groups of 3 (with the necessary padding) and convert them in octal. Then just "stick them together".

Let's take our number 38, it has the following representation:

$$
100\ 110_{bin}
$$

$100_{bin}$ converts to $4_{oct}$, while $110_{bin}$ converts to $6_{oct}$. If we stick them together we obtain the final result: $46_{oct}$.


### Gray Code

:::: wizardry ::::
Gray code isn't really used in normal situations, but it will be briefly explained since it will be used in [Karnaugh Maps](#karnaugh)
::::::::::::::::::

{{placeholder}}

<!-- TODO: Intro to gray code, very quick. -->

Basics of Logic
---------------

If we want our algorithms to be smart enough to be useful, we have to deal with conditionals. That's where logic comes in. In this section we will take a quick look at truth tables as well as logic operations.

### Truth tables

Truth tables are used to represent the output of a logic operation. It represents the inputs on the left side, while on the right side the result is shown.

Truth tables in this book will have the following look:

| A   | B   | **$f$** |
| :-: | :-: | :-----: |
| 0   | 0   | **0**   |
| 0   | 1   | **0**   |
| 1   | 0   | **1**   |
| 1   | 1   | **0**   |

### Common operators

{{placeholder}}

<!-- TODO: Explain common operators in binary logic -->

#### AND

The "AND" operator is a binary operator that outputs 1 when both inputs are 1. Here is its truth table:

| A   | B   | **AND** |
| :-: | :-: | :-----: |
| 0   | 0   | **0**   |
| 0   | 1   | **0**   |
| 1   | 0   | **0**   |
| 1   | 1   | **1**   |

This operator is used to express conditionals where you want two conditions to be true at the same time.

#### OR

The "OR" operator (sometimes called "inclusive or", as opposed to the XOR operator) is a binary operator that outputs 1 when either of the inputs is 1, including the case when both are 1. Here is its truth table:

| A   | B   | **OR**  |
| :-: | :-: | :-----: |
| 0   | 0   | **0**   |
| 0   | 1   | **1**   |
| 1   | 0   | **1**   |
| 1   | 1   | **1**   |

This operator is used to express conditionals where you want at least one condition to be true.

#### NOT

The "NOT" operator is a unary operator that takes a single input and "inverts" it. That means that if the input is 1, the "NOT" operator will output 0, if the input is 0 the "NOT" operator will output 1 instead.

Here is its truth table:

| A   | **NOT** |
| :-: | :-----: |
| 0   | **1**   |
| 1   | **0**   |

#### XOR

The "XOR" operator (called "exclusive or") is an operator that takes two input and outputs 1 when only one of the two inputs is 1. If both inputs have the value 1, the "XOR" operator will output 0.

Here is its truth table:

| A   | B   | **XOR** |
| :-: | :-: | :-----: |
| 0   | 0   | **0**   |
| 0   | 1   | **1**   |
| 1   | 0   | **1**   |
| 1   | 1   | **0**   |

This operator is used when you want to express conditionals where only one of the two inputs is true.

### Logic operations vs bitwise operations

{{placeholder}}

<!-- TODO: Difference between normal logic operations and bitwise operations -->

### De Morgan's Laws and Conditional Expressions

De Morgan's laws are fundamental in computer science as well as in any subject that involves propositional logic. We will take a quick look at the strictly coding-related meaning.

De Morgan's laws can be written as:

> not (A and B) = not A or not B
>
> not (A or B) = not A and not B

In symbols:

$$ \overline{(A \land B)} = \bar{A} \lor \bar{B} $$
$$ \overline{(A \lor B)} = \bar{A} \land \bar{B} $$

These laws allow us to express our own conditionals in different ways, allowing for more readability and maybe avoid some boolean manipulation that can hinder the performance of our game.

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

In any project, the ability to code quickly is as important as the performance of the final product: there is a thin balance to strike between "having a product with good performance" and "having a product that is released when needed". If your product releases too late, it doesn't matter how performant it is, the market will have chosen another product. If your product releases early but it underperforms, it will be replaced by better products.

Thus some hybrid approaches have been invented: one of these is, for instance, bytecode-compiled languages.

Bytecode-compiled languages (sometimes called "Languages with intermediate representation") are something that is not quite compiled, but it's not precisely interpreted either: the code is converted into bytecode, which is then fed to the interpreter (or "virtual machine") to run.

Being a representation that is "closer to the hardware" than the original source code, there is a gain in performance, while keeping the flexibility of interpreted code.

:::: trivia ::::
Some programming languages, like Haskell and Vala use the C programming language as an intermediate language, since C was meant to be an abstraction of the assembly language.
::::::::::::::::

Other approaches include Just-In-Time compiling, which trades off some longer starting times (sometimes called "warmup times") for better overall performance.

Among the bytecode-compiled languages we can find Java and Python, while Lua can be considered a Just-In-Time Compiled language (thanks to LuaJIT).

#### By Paradygm

A programming paradygm is how the programming language lets you program. There is not a single, definitive way to code, thus programming languages can be distinguished by their paradygm.

##### Imperative Languages

Imperative languages are probably the most spread in modern programming: they make use of "orders" (called "statements") to change the status of the program.

This paradygm makes use of variables, statements, subroutines to make the program look like a set of instructions, a recipe, to make the program do what it needs to do (an algorithm).

Imperative languages include C, COBOL, Basic and Fortran.

##### Functional Languages

Functional languages make programs work by applying and composing functions (in the mathematical sense). Functions can be bound to variables and chained together (composed) to reach the result.

Functional languages include Haskell, Common Lisp and Scheme.

##### Multiparadigm Languages

Many programming languages tend to "meld together" many programming paradygms, allowing (for instance) for functional style programming in imperative languages.

This means that functions can be bound to variables and passed around as any other object, they can be composed to reach the result if the programmer decides to do so (for instance for readability).

Multiparadigm languages include Python, Lua and Go.

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

- **C++**, a compiled programming language with strong static typing. It is multiparadigm (although it was born as an imperative language) and has no garbage collector.
- **JavaScript**, an interpreted language (although some engines support Just-In-Time compiling), with weak dynamic typing that supports some duck typing principles. It is multiparadigm and features a garbage collector.
- **Lua**, a bytecode-compiled (or Just-In-Time compiled) language, with strong dynamic typing that supports some duck typing principles. It is multiparadigm and garbage-collected.
- **Python**, a bytecode-compiled language, with strong duck typing. It is multiparadigm and garbage-collected.

<!-- TODO: Add more languages as more editions come out -->

Recursion
---------

Starting from the (arguably hard) theme of recursion may seem weird, but it is important to understand recursion as soon as possible so we can make the best use of it.

There is a joke I like telling around about recursion:

> To understand recursion, you must first understand recursion.

What is recursion? Recursion is the usage of a function that calls itself.

Your first question will probably be: wouldn't that make the program lock up forever in some kind of loop? It may. But if you're careful, recursion is an amazing tool that allows you to earn a lot of clarity and brevity.

Let's imagine a simple algorithm: we want to make our program count backwards from a number `n` to 0. In a simple "loop" fashion, we may write the following:

```{src='computer_science/recursion_loop' caption='Counting from n to 0 using a loop'}
```

Pretty simple, right? A real-world example would be counting back from 10 to 0: we print 10, we subtract 1 to get 9, we print 9, subtract 1 to get 8, ...

Let's turn our thinking around for a second. We can see counting back from to 10 to 0 like this: we print 10 and then we count backwards from 9. Counting backwards from 9 would just mean printing 9 and then counting back from 8, etc...

We just turned our simple loop into a recursive function:

```{src='computer_science/recursion' caption='Counting from n to 0 using recursion'}
```

Recursive functions have three main components:

- A **base case** (sometimes called a "stop condition"): this allows the function to stop calling itself when a certain condition is reached;
- A **procedure** that elaborates on data or simply does something (in our example, it just prints the number);
- A **recursive call** to the same function we are writing, the call is done in a way that every call gets closer to the "stop condition". It can be done by calling the function on a subset of its argument (if it is a list), until the list has only 1 item or on a smaller number (if the function argument is a number instead).

Recursion can be classified in many ways:

- **By the number of recursive calls:** single vs multiple recursion;
- **By how the recursive call is made:** direct (a function calls itself directly) vs indirect (a function A is called by another function B, which in turn is called by function A)
- **By the position of the recursive call:** head vs tail recursion.

I want to underline the last distinction: what we've seen in the previous listing is called "tail recursion": the recursive call is done **after** everything else (the procedure).

Head recursion is instead done when the recursive call is done **before** the procedure starts, so we can transform our "count down" function to a "count up" just by switching from "tail" to "head" recursion and adding a print statement.

```{src='computer_science/head_recursion' caption='Counting from 0 to n using head recursion'}
```

Computers are (not) precise {#precision_issues}
---------------------------

There are many differences between humans and computers, among those there is one that will keep haunting us in our journey: humans make calculations in "base 10" (decimal), computers make calculation in "base 2" (binary).

This requires computers to represent numbers differently, usually with the exponent+fraction representation (IEEE 754). Also computers have limited resources, thus have no concept of "infinity" (and conversely of "infinitesimal").

Let's assume a computer with a fixed (and reduced) precision and we execute the following C++ program (you can just copy it verbatim):

```{.cpp caption="A simple float precision test"}
#include <iostream>
#include <iomanip>

int main ()
{
    // This will reduce and fix the computer's precision for this execution
    std::cout << std::setprecision(20);

    float d1(1.0);
    std::cout << "This should be 1.0: " << d1 << std::endl;

    float d2(0.1);
    std::cout << "This should be 0.1: " << d2 << std::endl;

    float d3(0.1*0.1);
    std::cout << "This should be 0.01:" << d3 << std::endl;

    bool x (0.1 + 0.1 + 0.1 == 0.3);
    std::cout << "This should be true (1): " << x << std::endl;

    return 0;
  }
```

We save it as "precision_test.cpp" and compile it with the following command line (on Linux):

```{.sh}
g++ -Wall -Wextra -Werror -O0 precision_test.cpp -o precision_test.bin
```

This program will temporarily set a reduced precision in our number representation, and try to output the values of the numbers $1$, $0.1$ and $0.1^2=0.01$, let's see the results:

![Results of the simple float precision test](./images/computer_science/precision_1.png){width=50%}

With the number $1$ it's all good, but... what is going on with $0.1$? What is all that garbage? The number $0.01$ is even worse! That's not even close! Why $0.1 + 0.1 + 0.1$ comes out as not $0.3$! **What is maths anymore?**

We have just met one of the (many) limitations of computers: computers cannot represent certain numbers without "approximating". Compilers and libraries exist to work around these issues, but we need to be ready to avoid surprises.

Just to reiterate: this is not a problem of the single programming language, we can see that C++ is affected, but also Python has the same issue:

![Python 2 has the same issues with precision as C++](./images/computer_science/precision_2.png){width=50%}

![Python 3 doesn't fare much better when it comes to precision](./images/computer_science/precision_3.png){width=50%}

This is a computer issue in general: this may not be a huge problem for general use but, if we try to be too precise with our calculations, this may come back to bite us.


### Catastrophic cancellation

:::: wizardry ::::
Catastrophic cancellation is one of the many pitfalls that you may encounter when dealing with very small numbers. This doesn't happen really often in the world of game development, feel free to just skim thrhough this mostly informative section.
::::::::::::::::::

With a name as dangerous-sounding as "catastrophic cancellation", this sure looks like a dangerous phenomenon, but it's only dangerous if we don't know what it is.

Catastrophic Cancellation (sometimes called "cancellation error") is an event that may happen when subtracting two (usually large) numbers that are close to each other in value.

**Warning:** from here on, in this section, there will be some technical language. I will try to make it as simple and understandable as possible.

Let's imagine a computer, such computer's memory can handle at most 8 decimals while its *[A.L.U.~\[g\]~](#gl_alu)* (the unit that takes case of "doing maths") can handle at most 16 decimal places.

Now let's take two numbers:

$$x=0.5654328749846\ y=0.5654328510104$$

When we transfer such numbers in our memory, the computer will approximate such numbers to fit in its memory constraints. We'll represent that by applying to each number a function $fl()$ that we can read as "float representation of this number". So we'll end up having:

$$fl(x)=0.56543287\ fl(y)=0.56543285$$

This is generally called an "assignment error", where during the assignment to a variable, a number loses part of its information.

Let's try an calculate how off those approximations are (by calculating the percent "relative error"), just to get an idea of what we lost by just loading the numbers on our "fake computer":

$$\delta_x = \frac{| x - fl(x) |}{x} = ~ 0.00000088\%$$
$$\delta_y = \frac{| y - fl(y) |}{y} = ~ 0.00000017\%$$

We can see that our approximations are **very close** to the numbers we want to calculate, now let's calculate $x-y$. Making things by hand we would have:

$$x - y = 0.239772 \times 10^{-7}$$

That's a tiny number right there. Now let's calculate $fl(x) - fl(y)$, remembering that the A.L.U. will fill up to 16 decimals:

$$fl(x) - fl(y) = 0.5654328700000000 - 0.5654328500000000 = 0.0000000200000000 = 0.2 \times 10^{-7}$$

That doesn't look so bad, unless we look at the "relative error":

$$\delta = \frac{| 0.239772 \times 10^{-7} - 0.2 \times 10^{-7} |}{0.239772 \times 10^{-7}} = 16.6\%$$

We are off by 16\% of the total result, this is actually really bad.

What happened? If you look closely, the numbers are really close and even have 7 decimal digits in common, since our computer can memorize only 8 digits, the 9th to 13th decimal digits that looked so unimportant suddenly become a huge part of the result (due to the subtraction) but are already lost.

Random Numbers are not really random {#random}
------------------------------------

Computers are deterministic machines, given the same set of instructions and inputs, they will **always** return the same output. Someone may think about "random number generators" and sure, those programs look like they spit random numbers on your screen, but they actually don't.

The most important number when generating random numbers is called *seed* and it's the number used by the *generator* to produce random numbers.

Let's see an example of a random number generator in C++ (you can copy this program verbatim to try it):

```{.cpp caption="A simple random number generation program"}
#include<iostream>
int main(){
    // First of all we get the seed
    unsigned int seed;
    std::cout << "Type the seed: " ;
    std::cin >> seed;
    // Now we seed the randomizer
    srand(seed);
    // Small presentation
    std::cout << "This generator will now generate 10 random numbers" << std::endl;
    // Output 10 random numbers
    for (int i = 0; i < 10; ++i) {
        std::cout << rand() << std::endl;
    }
    // Finish the program
    return 0;
}
```

We can save this program as `random_seed.cpp` compile this program with the following command:

```{.sh}
g++ -Wall -Wextra -Werror -O0 random_seed.cpp -o random_seed.bin
```

When we run the program, it will ask us to input a seed (which in our case is a number), after that it will just print 10 random numbers based on that seed. What would happen if we ran the program twice and use the same seed?

![Running a random number generator with the same seed will always output the same numbers](./images/computer_science/rng_seed.png){width=40%}

Random numbers generated by computers are never truly random, that's why they are more properly called "pseudo-random numbers".

### How to seed a random number generator

From what we have seen earlier, the seed of our random number generator is something we need to be mindful about.

Choosing a static seed will make our game completely deterministic (if played in the same conditions), like we didn't use random numbers at all.

Some games use internal timers to see the random number generator, be it the time that the game has been running, the time that has passed from the beginning of the mission or something similar. This allows you to have some kind of "controlled RNG" that still has a bit of reproducibility.

Some choices expose the game to the possibility of RNG manipulation: where the player has partial or total control over the random number generator, by performing specific actions at specific times, for instance.

A very easy way to seed a generator is using the system time. Here's a more advanced random number generator that uses system time as its seed: if you run the program reasonably slow (not quicker than once a second) you will see the numbers changing.

```{.cpp caption="A random number generation program that uses system time as seed"}
#include<iostream>
#include<time.h>
int main(){
    // First of all we get the seed
    unsigned int seed = time(nullptr);
    // We print it for reference
    std::cout << "The current seed is:" << seed << std::endl;
    // Now we seed the randomizer
    srand(seed);
    // Small presentation
    std::cout << "This generator will now generate 10 random numbers" << std::endl;
    // Output 10 random numbers
    for (int i = 0; i < 10; ++i) {
        std::cout << rand() << std::endl;
    }
    // Finish the program
    return 0;
}
```

We can save this program as `rand.cpp` compile this program with the following command:

```{.sh}
g++ -Wall -Wextra -Werror -O0 rand.cpp -o rand.bin
```

This is the result of the program being run twice, one second apart:

![Using the system time as RNG seed guarantees a degree of randomness](./images/computer_science/rng_time.png){width=40%}
