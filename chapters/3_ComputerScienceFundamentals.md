\null\clearpage
Some Computer Science Fundamentals
===================================

In order to understand some of the language that is coming up, it is necessary to learn a bit of the computer science language and fundamentals.

This chapter will briefly explain some of the language and terms used, their meaning and how they contribute to your activity of developing games.

In this chapter we'll assume you already know what the following terms mean:

- Truth Table
- Algorithm

Estimating the order of algorithms
---------------------------------------

Now more than ever, you need to be able to be efficient. How do you know how "efficient" some piece of algorithm is?

Seeing how much time it takes is not an option, computer specifications change from system to system, so we need something that could be considered "cross-platform".

This is where notations come into play.

There are 3 types of Asymptotic notation you should know: $\Omega$, $\Theta$ and O.

**$\Omega$()** represents **a lower bound**: this means that the algorithm will take **at least** as many cycles as specified.

**O()** represents **an upper bound**: it's the most used notation and means that the algorithm will take **at most** as many cycles as specified.

**$\Theta$()** is a **tight bound**, used when the big-O notation and the big-$\Omega$ notation have the same value, which can help define the behaviour of the algorithm better.

We will now talk about the most common Big-O notations, from "most efficient" to "least efficient".

### O(1)

An algorithm that executes in **O(1)** is said to execute "in constant time", which means that no matter how much data is input in the algorithm, said algorithm will execute in the same time.

An example of a simple O(1) algorithm is an algorithm that, given a list of elements (with at least one element), returns `True` if the first element is `null`.

\code{computer_science/o1}

To be precise, this algorithm will perform both in O(1) and $\Omega(1)$, so it will perform in $\Theta(1)$.

### O(log(n))

An algorithm that executes in O(log(n)) is said to execute in "logarithmic time", which means that given an input of **n** items, the algorithm will execute **log(n)** cycles at most.

An example of a O(log(n)) algorithm is the so-called "binary search" on a ordered list of items.

\code{computer_science/binary_search}

The best case is the time when you get the element to find to be the "middle element" of the list, in that case the algorithm will execute in linear time: $\Theta(1)$ - You need **at least one lookup** ($\Omega(1)$) and **at most one lookup** ($O(1)$).

In the worst case, the element is not present in the list, so you have to split the list and find the middle element until you realize that you don't have any more elements to iterate - this translates into a **tight bound** of $\Theta(log_{2}n)$

### O(n)

An algorithm that executes in O(n) is said to execute in "linear time", which means that given an input of **n** items, the algorithm will execute at most **n** cycles.

An example of a simple O(n) algorithm is the one that prints a list, element by element.

\code{computer_science/printlist}

It's evident that this algorithm will call the `print` function `n` times, where `n` is the size of the list. This translates in a $\Theta(n)$ complexity, which is both $O(n)$ and $\Omega(n)$.

There is no "best" or "worst" case here, the algorithm prints `n` elements, no matter their order, the alignment of planets and stars or the permission of its parents.

### O(n·log(n))

An algorithm that executes in O(n·log(n)) executes in a time slightly longer than a linear algorithm, but it's still considered "ideal". These algorithms are said to execute in "quasi-linear", "log-linear", "super-linear" or "linearithmic" time.

Given an input of **n** elements, these algorithms execute **n·log(n)** steps, or cycles.

Some algorithms that run in O(n·log(n)) are:

- Quick Sort
- Heap Sort
- Fast Fourier Transforms (FFT)

These algorithms are more complex than a simple example and would require a chapter on their own, so we'll leave examples aside for now.

### O(n^2^)

Quadratic algorithms, as the algorithms that execute in O(n^2^) are called, are the door to the "danger zone".

These algorithms can eat your CPU time quite quickly, although they can still be used for small computations somewhat efficiently.

Given an input of **n** elements, these algorithms execute **n^2^** cycles, which means that given an input of **20** elements, we'd find ourselves executing **400** cycles.

A simple example of a quadratic algorithm is "bubble sort". A pseudo-code implementation is written here.

\code{computer_science/bubblesort}

Anything with complexity higher than O(n^2^) is usually considered unusable.

### O(2^n^)

Algorithms that execute in exponential time are considered a major code red, an will usually be replaced with heuristic algorithms (which trade some precision for a lower complexity).

Given an input of `n` elements, an algorithm that executes in O(2^n^) will execute 2^20^ = 1 048 576 cycles!


A primer on calculating the order of your algorithms
-----------------------------------------------------

\placeholder

<!-- TODO: Teach people how to estimate their algorithms -->

Simplifying your conditionals with Karnaugh Maps
------------------------------------------------

Karnaugh maps are a useful tool to simplify boolean algebra expressions, as well as identifying and potentially solving race conditions.

The output of a Karnaugh Map will always be an "OR of ANDs".

The best way to explain them is to give an example.

Let's take the following truth table:

| A | B |  $f$  |
|:-:|:-:|:-----:|
| 0 | 0 | $0$   |
| 0 | 1 | $1$   |
| 1 | 0 | $1$   |
| 1 | 1 | $0$   |

: The first truth table we'll simplify with Karnaugh Maps

Said table can contain any number of variables (we'll see how to implement those). To be precise, this table represents the formula `A XOR B` (XOR means 'exclusive or').

Let's arrange it into a double-entry table, like this (Values of A are on top, values of B are on the left):

\begin{table}[H]
    \centering
    \caption{Karnaugh Map for A XOR B}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & 1 & 0\\
        \hline
    \end{tabular}
\end{table}

Now we have to identify the biggest squares or rectangles that contain 2^n^ elements equal to 1 so that we can cover all the "1" values we have (they can overlap). In this case we're unlucky as we have only two small rectangles that contain one element each:

\begin{table}[H]
    \centering
    \caption{Karnaugh Map where the elements of the two "rectangles" have been marked green and red}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & \textcolor{red}{1} \\
        & \textbf{1} & \textcolor{green}{1} & 0\\
        \hline
    \end{tabular}
\end{table}

In this case, we have the result we want with the following formula: $f = (A \land \bar{B}) \lor (\bar{A} \land B$)

Not an improvement at all, but that's because the example is a really simple one.

### "Don't care"s

Karnaugh Maps show more usefulness when we have the so-called "don't care"s, situations where we don't care (wow!) about the result. Here's an example.

| A | B |  $f$  |
|:-:|:-:|:-----:|
| 0 | 0 | $0$   |
| 0 | 1 | $1$   |
| 1 | 0 | $1$   |
| 1 | 1 | $x$   |

: Truth table with a "don't care" value

Putting this truth table into a Karnaugh map we get something a bit more interesting:

\begin{table}[H]
    \centering
    \caption{Karnaugh Map with a "don't care" value}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & 1 & x\\
        \hline
    \end{tabular}
\end{table}

Now we have a value that behaves a bit like a "wild card", that means we can pretend it's either a 0 or 1, depending on the situation. In this example we'll pretend it's a 1, because it's the value that will give us the biggest "rectangles".

\begin{table}[H]
    \centering
    \caption{Karnaugh Map where we pretend the "don't care" value is equal to 1}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & 1 & 1\\
        \hline
    \end{tabular}
\end{table}

Now we can find two two-elements rectangles in this map.

The first is the following one:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & \textcolor{red}{1} & \textcolor{red}{1}\\
        \hline
    \end{tabular}
\end{table}

In this case, we can see that the result is 1 when $B=1$, no matter the value of A. We'll keep this in mind.

The second rectangle is:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & \textcolor{green}{1} \\
        & \textbf{1} & 1 & \textcolor{green}{1}\\
        \hline
    \end{tabular}
\end{table}

In this case, we can see that the result is 1 when $A=1$, no matter the value of B.

This translates into a formula of: $f= (A) \lor (B)$, considering that we don't care about the result that comes out when $A=1$ and $B=1$.

### A more complex map

When we have more variables, like the following truth table:

| A | B | C | D |  $f$  |
|:-:|:-:|:-:|:-:|:-----:|
| 0 | 0 | 0 | 0 |  $0$  |
| 0 | 0 | 0 | 1 |  $0$  |
| 0 | 0 | 1 | 0 |  $0$  |
| 0 | 0 | 1 | 1 |  $0$  |
| 0 | 1 | 0 | 0 |  $0$  |
| 0 | 1 | 0 | 1 |  $0$  |
| 0 | 1 | 1 | 0 |  $1$  |
| 0 | 1 | 1 | 1 |  $0$  |
| 1 | 0 | 0 | 0 |  $1$  |
| 1 | 0 | 0 | 1 |  $1$  |
| 1 | 0 | 1 | 0 |  $1$  |
| 1 | 0 | 1 | 1 |  $1$  |
| 1 | 1 | 0 | 0 |  $1$  |
| 1 | 1 | 0 | 1 |  $1$  |
| 1 | 1 | 1 | 0 |  $1$  |
| 1 | 1 | 1 | 1 |  $x$  |

Now we'll have to group up our variables and put them in a Karnaugh Map using Gray Code, practically each row or column differs from the adjacent ones by only one bit.

The resulting Karnaugh map is the following (AB on columns, CD on rows):

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & x & 1\\
        & \textbf{10} & 0 & 1 & 1 & 1\\
        \hline
    \end{tabular}
\end{table}

We can see two rectangles that contain 2^n^ items, one with 2 items, the other with 8, considering the only "don't care" value as 1.

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & \textcolor{red}{1} & \textcolor{red}{1}\\
        & \textbf{01} & 0 & 0 & \textcolor{red}{1} & \textcolor{red}{1}\\
        & \textbf{11} & 0 & 0 & \textcolor{red}{x} & \textcolor{red}{1}\\
        & \textbf{10} & 0 & 1 & \textcolor{red}{1} & \textcolor{red}{1}\\
        \hline
    \end{tabular}
\end{table}

In this first rectangle, we can see that the values of C and D don't matter towards the result, as well as the value of B. The only variable that gives the result on this rectangle is $A=1$. We'll keep that in mind

Let's see the second rectangle:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & x & 1\\
        & \textbf{10} & 0 & \textcolor{green}{1} & \textcolor{green}{1} & 1\\
        \hline
    \end{tabular}
\end{table}

In this case A doesn't give any contribution to the result, but at the same time we need $B=1$, $C=1$ and $D=0$ to get the wanted result.

$D=0$ translates into $\bar{D}=1$, which brings the formula to: $f = A \lor (B \land C \land \bar{D})$.

If we didn't have that "don't care" value, everything would have been more complex.

### Guided Exercise

Let's remove the "don't care" value and have the following truth table:

| A | B | C | D |  $f$  |
|:-:|:-:|:-:|:-:|:-----:|
| 0 | 0 | 0 | 0 |  $0$  |
| 0 | 0 | 0 | 1 |  $0$  |
| 0 | 0 | 1 | 0 |  $0$  |
| 0 | 0 | 1 | 1 |  $0$  |
| 0 | 1 | 0 | 0 |  $0$  |
| 0 | 1 | 0 | 1 |  $0$  |
| 0 | 1 | 1 | 0 |  $1$  |
| 0 | 1 | 1 | 1 |  $0$  |
| 1 | 0 | 0 | 0 |  $1$  |
| 1 | 0 | 0 | 1 |  $1$  |
| 1 | 0 | 1 | 0 |  $1$  |
| 1 | 0 | 1 | 1 |  $1$  |
| 1 | 1 | 0 | 0 |  $1$  |
| 1 | 1 | 0 | 1 |  $1$  |
| 1 | 1 | 1 | 0 |  $1$  |
| 1 | 1 | 1 | 1 |  $0$  |

Let's put it into a Karnaugh Map:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & 0 & 1\\
        & \textbf{10} & 0 & 1 & 1 & 1\\
        \hline
    \end{tabular}
\end{table}

Find the biggest rectangles:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & \textcolor{red}{1}\\
        & \textbf{01} & 0 & 0 & 1 & \textcolor{red}{1}\\
        & \textbf{11} & 0 & 0 & 0 & \textcolor{red}{1}\\
        & \textbf{10} & 0 & 1 & 1 & \textcolor{red}{1}\\
        \hline
    \end{tabular}
\end{table}

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & \textcolor{green}{1} & \textcolor{green}{1}\\
        & \textbf{01} & 0 & 0 & \textcolor{green}{1} & \textcolor{green}{1}\\
        & \textbf{11} & 0 & 0 & 0 & 1\\
        & \textbf{10} & 0 & 1 & 1 & 1\\
        \hline
    \end{tabular}
\end{table}


\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & 0 & 1\\
        & \textbf{10} & 0 & \textcolor{purple}{1} & \textcolor{purple}{1} & 1\\
        \hline
    \end{tabular}
\end{table}

Extract the result: $f = (A \land \bar{C}) \lor (A \land \bar{B}) \lor (B \land C \land \bar{D})$

Object Oriented Programming
----------------------------

### Introduction

One of the biggest programming paradigms in use is surely the "Object Oriented Programming" (from now on: "OOP") paradigm. The fundamental unit of a program, in this paradigm is the *Object*. This paradigm allows to structure your code in a more modular and re-usable way, as well as implementing abstractions, allowing for more solid code and making it possible for other code to make use of your own code without needing to know any details besides it *Interface*.

### Objects

Objects are the fundamental unit in OOP, objects are essentially a collection of data and functions. Objects are actually the physical instantiation of what is called a "Class".

To simplify the concept: a "Class" is a house blueprint, an "Object" is the house itself.

Objects contain data and functions, for the sake of precision, we will use their technical names:

- Functions that are part of an object are called **methods** and they can be classified as:
    - *Instance Methods* when they act on a single object instance;
    - *Static Methods* when they don't (usually they're utility functions), that also means that these methods belong to the Class itself and not to its instance.
- Each piece of data contained in the class is called a **Field** and they can be classified as:
    - *Instance Fields* when they're part of the instance and can change from instance to instance;
    - *Static Fields* when they're part of the class but don't change between instances (**Caution:** it does not mean they cannot change, in that case the change will snowball into all the instances).

### Abstraction and Interfaces

Abstraction is a fundamental point in OOP, and it is usually taken care of via so-called **Interfaces**.

Interfaces are the front-end that an object offers to other objects so they can interact.

As an example: the interface to your PC is given by Keyboard, Mouse and Screen - you don't need to know how the single electron travels through the circuits to be able to use a computer; same goes for a class that offers a well-abstracted interface.

Being able to abstract a concept, removing the necessity to know the internal workings of the code that is used, is fundamental to be able to write solid and maintainable code, because implementations change, but interfaces rarely do.

Making classes work together with interfaces allows you to modify and optimize your code without having each edit snowball into a flurry of compiler (or interpreter) errors. For instance: a rectangle class exposes in its interface a method `getArea()` - you don't need to know how to calculate the area, you just call that method and know it will return the area of the rectangle.

The concept of keeping the internal workings of a class is called *Information Hiding*.

### Inheritance and Polymorphism

One of the biggest aspects of OOP is **inheritance**: you can create other classes based on a so-called "base class", allowing for extendability of your software.

You can create a "Person" class, with a name, surname and age as fields, and by inheriting from the "Person" class you can create a "Student" class, which has all the fields from Person, plus the "clubs" and "grade" fields.

This allows to create a "tree of classes" that represents part of your software.

From inheritance, OOP presents a concept called **Polymorphism** (From "Poly" - Many, "Morph" - Shape), where you can use the base class to represent the entire class tree, allowing for substitution.

In our "Person-Student" example, you could use a pointer to either a Person or a Student for the sake of getting their first name.

\placeholder

### Composition

\placeholder

### "Composition over Inheritance" design

\placeholder

### Coupling

\placeholder

### SOLID Principles

SOLID is a mnemonic acronym that condenses five principles of good design, to make code and software that is understandable, flexible and maintainable.

- **Single Responsibility**: Each class should have a single responsibility, it should take care of one part of the software specification and each change to said specification should affect only said class. This means you should avoid the so-called "God Classes", classes that take care of too much, know too much about the system and in a nutshell: have too much responsibility in your software.
- **Open-closed Principle**: Each software entity should be open to extension, but closed for modification. This means that each class (for instance) should be extensible, either via inheritance or composition, but it should not be possible to modify the class's code. This is practically enforcing *Information Hiding*.
- **Liskov Substitution Principle**: Objects in a program should be replaceable with instances of their subtypes and the correctness of the program should not be affected. This is the base of inheritance and polimorphism, if by substituting a base class with one of its child (which should have a Child-is-a-Base relationship, for instance "Circle is a shape") the program is not correct anymore, either something is wrong with the program, or the classes should not be in a "IS-A" relationship.
- **Interface Segregation**: Classes should provide many specific interfaces instead of one general-purpose interface, this means that no client should depend on methods that it doesn't use. This makes the software easier to refactor and maintain, and reduces coupling.
- **Dependency Inversion**: Software components should depend on abstractions and not concretions. This is another staple of nutshell programming and OOP - Each class should make use of some other class's interface, not its inner workings. This allows for maintainability and easier update and change of code, without having the changes snowball into an Armageddon of errors.

<!-- TODO: a lean introduction to the concept of objects and abstraction -->

Designing entities as data
-------------------

Sometimes it can be useful to design your entities as data, instead of making them into static objects that possibly require a new release of your product.

Designing your objects as data allows you to use configuration files to create, configure, tinker and extend your product, as well as allow for modifications by people who are fans of your game.

For instance, in a fantasy RPG you could have 3 types of enemies all defined as classes:

- Skeleton
- Zombie
- Ghost Swordsman

Which all have the same behaviour but different animations and sprites.

These classes can inherit from an "entity" abstract class which defines the base behaviour and then can be extended to create each unique enemy.

Another idea could be designing an "entity" class that can be instantiated, and have a configuration file that defines what each entity is and what its properties are.

An idea could be the following

~~~~.yaml
entity:
  name: skeleton
  health: 10
  damage_on_hit: 2.5
  spritesheet: "./skelly.png"
  animations:
    walking:
      start_sprite: 4
      frame_no: 4
      duration: 0.2
    attacking:
      start_sprite: 9
      frame_no: 2
      duration: 0.1
~~~~

With more complex building algorithms, it is possible to change behaviours and much more with just a configuration file, and this gives itself well to roguelike games, which random selection of enemies can benefit from an extension of the enemy pool. In fact, it's really easy to configure a new type of enemy and have it work inside the game without recompiling anything.

This allows for more readable code and a higher extensibility.

Reading UML diagrams
--------------------

<!-- TODO: Give a primer on how to read UML diagrams -->

### Use Case Diagrams

\placeholder

### Class Diagrams

\placeholder

### Activity Diagrams

\placeholder
