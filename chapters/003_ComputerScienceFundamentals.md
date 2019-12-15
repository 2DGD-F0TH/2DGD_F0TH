\null\clearpage

Some Computer Science Fundamentals
===================================

\epigraph{The computing scientist's main challenge is not to get confused by the complexities of his own making.}{\textit{Edsger W. Dijkstra}}

In order to understand some of the language that is coming up, it is necessary to learn a bit of the computer science language and fundamentals.

This chapter will briefly explain some of the language and terms used, their meaning and how they contribute to your activity of developing games.

In this chapter we'll assume you already know what the following terms mean:

- Truth Table
- Algorithm

De Morgan's Laws and Conditional Expressions
----------------

De Morgan's laws are fundamental in computer science as well as in any subject that involves propositional logic. We will take a quick look at the strictly coding-related meaning.

De Morgan's laws can be written as:

> not (A and B) = not A or not B
>
> not (A or B) = not A and not B

In symbols:

$$ \overline{(A \land B)} = \bar{A} \lor \bar{B} $$
$$ \overline{(A \lor B)} = \bar{A} \land \bar{B} $$

These laws allow us to express our own conditionals in different ways, allowing for more readability and maybe avoid some boolean manipulation that can hinder the performance of our game.

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

\code{computer_science/o1}{Example of an O(1) algorithm}

To be precise, this algorithm will perform both in O(1) and $\Omega(1)$, so it will perform in $\Theta(1)$.

### O(log(n))

An algorithm that executes in O(log(n)) is said to execute in "logarithmic time", which means that given an input of **n** items, the algorithm will execute **log(n)** cycles at most.

An example of a O(log(n)) algorithm is the so-called "binary search" on a ordered list of items.

\code{computer_science/binary_search}{Example of an O(log(n)) algorithm (Binary Search)}

The best case is the time when you get the element to find to be the "middle element" of the list, in that case the algorithm will execute in linear time: $\Theta(1)$ - You need **at least one lookup** ($\Omega(1)$) and **at most one lookup** ($O(1)$).

In the worst case, the element is not present in the list, so you have to split the list and find the middle element until you realize that you don't have any more elements to iterate - this translates into a **tight bound** of $\Theta(log_{2}n)$

### O(n)

An algorithm that executes in O(n) is said to execute in "linear time", which means that given an input of **n** items, the algorithm will execute at most **n** cycles.

An example of a simple O(n) algorithm is the one that prints a list, element by element.

\code{computer_science/printlist}{Example of an O(n) algorithm (printing of a list)}

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

\code{computer_science/bubblesort}{Example of an O(n²) algorithm (bubble sort)}

Anything with complexity higher than O(n^2^) is usually considered unusable.

### O(2^n^)

Algorithms that execute in exponential time are considered a major code red, an will usually be replaced with heuristic algorithms (which trade some precision for a lower complexity).

Given an input of `20` elements, an algorithm that executes in O(2^n^) will execute 2^20^ = 1 048 576 cycles!


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

In some languages it is possible for an object to inherit from multiple other objects, this is called "Multiple Inheritance"

### The Diamond Problem

Usually when you call a method that is not present in the object itself, the program will look through the object's parents for the method to execute. This usually works well when there is no ambiguity, but what if it happens?

When multiple inheritance is involved, there is a serious possibility of a situation similar to the following

![Example of a diamond problem](./images/computer_science/diamond.png){width=20%}

In this example, class A implements a method `dostuff()` that is overrode by both classes B and C (which inherit from A): now class D inherits from both B and C but does not override `dostuff()`, which one will be chosen?

This is the reason many languages do not implement multiple inheritance between classes (like Java, which allows multiple inheritance only between interfaces), other implement the so-called "virtual inheritance" (C++) and others again use an ordered list to solve the problem (Python).

This is not something you usually need to worry about, but you may want to be careful when you structure your classes to avoid "diamond problems", so to avoid headaches.

### Composition

As opposed to inheritance's "IS-A" relationship, composition makes use of a "HAS-A" type of relationship.

Composition allows to define objects by declaring which properties they have: a player character can be a sprite with a "Movable" component, or a box could have a "RigidBody" component.

This way we can create new objects by reusing basic components, making maintenance easier as well as saving lines of code, avoiding "the diamond problem" and reducing coupling.

\placeholder

### Coupling

Coupling is a term used to define the phenomenon where an edit to some part of a software snowballs into a bunch of edits in parts of the software that depend on the modified part, and the part that depend on the previously edited dependency, etc...

Introducing unnecessary coupling in our software will come back to bite us in the future, affecting maintainability in a very negative way, since any edit we make (for instance, to fix a bug) can potentially lead to edit the rest of the software (or game) we are writing.

This means that it's in our best interest to reduce code coupling as much as possible, following the good principles of "nutshell programming" and following the SOLID principles, shown next.

\placeholder

### The DRY Principle

DRY is a mnemonic acronym that stands for "Don't Repeat Yourself" and condenses in itself the principle of reducing repetition inside a software, replacing it with *abstractions* and by *normalizing data*.

This allows for each piece of code (and knowledge, since the DRY principle applies to documentation too) to be unambiguous, centralizing its responsibilities and avoiding repetition.

Violations of the DRY principle are called "WET" (write everything twice) solutions, which base themselves on repetition and give higher chances of mistakes and inconsistency.

### SOLID Principles

SOLID is a mnemonic acronym that condenses five principles of good design, to make code and software that is understandable, flexible and maintainable.

- **Single Responsibility**: Each class should have a single responsibility, it should take care of one part of the software specification and each change to said specification should affect only said class. This means you should avoid the so-called "God Classes", classes that take care of too much, know too much about the system and in a nutshell: have too much responsibility in your software.
- **Open-closed Principle**: Each software entity should be open to extension, but closed for modification. This means that each class (for instance) should be extensible, either via inheritance or composition, but it should not be possible to modify the class's code. This is practically enforcing *Information Hiding*.
- **Liskov Substitution Principle**: Objects in a program should be replaceable with instances of their subtypes and the correctness of the program should not be affected. This is the base of inheritance and polimorphism, if by substituting a base class with one of its child (which should have a Child-is-a-Base relationship, for instance "Circle is a shape") the program is not correct anymore, either something is wrong with the program, or the classes should not be in a "IS-A" relationship.
- **Interface Segregation**: Classes should provide many specific interfaces instead of one general-purpose interface, this means that no client should depend on methods that it doesn't use. This makes the software easier to refactor and maintain, and reduces coupling.
- **Dependency Inversion**: Software components should depend on abstractions and not concretions. This is another staple of nutshell programming and OOP - Each class should make use of some other class's interface, not its inner workings. This allows for maintainability and easier update and change of code, without having the changes snowball into an Armageddon of errors.

### "Composition over Inheritance" design

\placeholder

<!-- TODO: a lean introduction to the concept of objects and abstraction -->

Designing entities as data {#entitiesasdata}
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

```{.yaml caption="Example of an entity declared as data"}
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
```

With more complex building algorithms, it is possible to change behaviours and much more with just a configuration file, and this gives itself well to roguelike games, which random selection of enemies can benefit from an extension of the enemy pool. In fact, it's really easy to configure a new type of enemy and have it work inside the game without recompiling anything.

This allows for more readable code and a higher extensibility.

Reading UML diagrams
--------------------

UML (Universal Modeling Language) is a set of graphical tools that allow a team to better organize and plan a software product. Diagrams are drawn in such a way to give the reader an overall assessment of the situation described while being easy to read and understand.

In this chapter we will take a look at 3 diagrams used in UML:

- Use Case Diagrams
- Class Diagrams
- Activity Diagrams

### Use Case Diagrams

Use Case Diagrams are usually used in software engineering to gather requirements for the software that will come to exist. In the world of game development, use case diagrams can prove useful to have an "outside view" of our game, and understand how an user can interact with our game.

Here is an example of a use case diagram for a game:

![Example of a use case diagram](./images/computer_science/use_case.png){width=60%}

#### Actors {#UMLUserCaseActors}

Actors are any entity that can interface with our system (in this case, our game) without being part of it. Actors can both be human, machines or even other systems.

Actors are represented with a stick figure and can inherit from each other: this will create an "IS-A" relationship between actors.

![Example of an actor hierarchy](./images/computer_science/actors.png){width=20%}

In the previous example, we can see that a "Free User" is an "Authenticated User", as well as a "Power User" (which could be a paying user) is itself an "Authenticated User" while an "Ultimate User" (which could be a higher tier of paying user) is a "Power User" (thus has all the "Power User" capabilities, plus some unique) and by transitive property an "Authenticated User".

As seen, inheritance between actors is represented with a solid line with a hollow closed arrow. Such arrow points towards the "supertype" or "parent" from which the subject (or "subtype", or "child") inherits.

This representation will come back in the UML language for other diagrams too.

#### Use Cases

Use cases represent the functionalities that our system offers, and the relationships between them.

Use cases are represented with an ellipse with the name of the use case inside. Choosing the right name for a use case is extremely important, since they will represent the functionality that will be developed in our game.

![Example of a use case](./images/computer_science/use_case_single.png){width=15%}

##### Inheritance

As with many other elements used in UML, use cases can inherit from each other. Inheritance (also called "Generalization") is represented with a closed hollow arrow that points towards the parent use case.

![Example of a use case hierarchy](./images/computer_science/use_case_hierarchy.png){width=40%}

##### Extensions

Use case extensions specify how and when optional behaviour takes place. Extended use cases are meaningful on their own and are independent from the extending use case, while the extending use case define the optional behaviour that may not have much sense by itself.

Extensions are represented via a dashed line with an open arrow on the end, labeled with the `<<extend>>` keyword, pointing towards the extending use case.

![Example of a use case extension](./images/computer_science/use_case_extension.png){width=40%}

##### Inclusions

Inclusions specify how the behaviour of the included use case is inserted in the behaviour of the including use case. Inclusions are usually used to simplify large use cases by splitting them or extract common behaviours of two or more use cases.

In this situation, the including use case **is not** complete by itself.

Inclusions are represented via a dashed line with an open arrow on the end, labeled with the `<<include>>` pointing towards the included use case.

![Example of a use case inclusion](./images/computer_science/use_case_inclusion.png){width=40%}

#### Notes

In use case diagrams, as well as in many other UML diagrams, notes are used to jot down conditions, comments and everything useful to better understanding the diagram that cannot be conveyed through a well definite structure inside of UML.

Notes are shaped like a sheet of paper with a folded corner and are usually connected to the diagram with a dashed line. Each note can be connected to more than one piece of the diagram.

You can see a note at the beginning of this chapter, in the use case diagram explanation.

#### Sub-Use Cases

Use cases can be further detailed by creating sub-use cases, like the following example.

![Example of a sub-use case](./images/computer_science/subuse_case.png){width=40%}

### Class Diagrams

#### Classes

Class diagrams are used a step after analyzing your game, since they are used for planning classes. The central element of a class diagram is the "class", which is represented as follows:

![Example of classes in UML](./images/computer_science/class_example.png){width=60%}

Classes are made up by a class name, which is shown on top of the class; abstract classes are shown with a name in *italics*.

Public members are highlighted by a "+" symbol (or in our case, a green symbol) before their name, protected members use a "#" symbol (or a yellow symbol) and private members use a "-" symbol.

Static members are shown with an \underline{underlined} name, while abstract members are shown in *italics*.

#### Relationships between classes

Expressing only single classes on their own doesn't give UML a lot of expressive power when it comes to planning your games. Here we'll take a quick look at the most used relationships between classes.

![Relationships between classes in an UML Diagram](./images/computer_science/class_relationships.png){width=60%}

##### Inheritance

Inheritance is represented via a hollow closed arrow head that points towards the base class (exactly like in [Actor inheritance](#UMLUserCaseActors)), this means that the classes are in a "supertype and subtype" relationship.

![Example of inheritance in UML class diagrams](./images/computer_science/class_inheritance.png){width=15%}

In this example we say that "Student IS-A Person" and inherits all Person's methods and fields.

##### Association

Association represents a static relationship between two classes. This is usually represented with a solid line with an arrow. The arrow usually shows the reading order of the association, so if you see an "Author" class and a "Book" class, the "wrote" association will be pointing from the "Author" to the "Book" class.

In case the relationship is bi-directional, the arrow points are omitted, leaving only a solid line between the two classes.

![Example of association in UML class diagrams](./images/computer_science/class_association.png){width=15%}

An example of an association is the relationship between a "Person" and a "Magazine", such relationship is the "Subscription". In this case the relationship is bi-directional, since a "Magazine" can be subscribed by many people, but a single "Person" can subscribe to many "Magazine"s.

##### Aggregation and Composition

Aggregation is a special case of the association relationship, and represents a more specific case of it. Aggregation is a variant of a "has-a" relationship and represents a part-whole relationship.

Aggregation is represented with a hollow diamond and a line that points to the *contained* class, classes involved in an aggregation relationships *do not* have their life cycles dependant one another, that means that if the container is destroyed, the contained objects will keep on living. An example could be a teacher and their students, if the teacher dies the students will keep on living.

![Example of aggregation and composition in UML class diagrams](./images/computer_science/class_aggregation_composition.png){width=35%}

Composition is represented with a filled diamond instead than a hollow one, in this case there is a life cycle dependency, so when the container is destroyed the contents are destroyed too. Like when a university is dissolved, its departments will cease to exist.

##### Dependency

The dependency relationship is the one that gives us the least amount of coupling, it represents a "supplier-client" relationships, where the supplier supplies its functions (methods) to the client. The association is represented in a dashed line with an open arrow that points towards the supplier.

This means that the client class requires, needs or depends on the supplier.

There are many categories of dependency, like `<<create>` or `<<call>>` that explain further the type of dependency exists between the supplier and the client.

An example could be between a "Car Factory" and a class "Car": the "CarFactory" class depends on the "Car" class, and such dependency is an instantiation dependency.

![Example of dependency in UML class diagrams](./images/computer_science/class_dependency.png){width=15%}

#### Notes

As with Use Case diagrams, class diagrams can make use of notes too, and the graphical language used to represent them is exactly the same one used in the Use Case Diagrams.

#### Interfaces

Sometimes there is a need to convey the concept of "interface" inside a UML class diagram, that can easily be done in 2 ways:

- By using the class construct, with the keyword (called "stereotype") `<<interface>>` written on top of it;
- By using the "lollipop notation" (called "interface realization").

![Defining an interface in UML](./images/computer_science/interface.png){width=15%}

![Interface Realization in UML](./images/computer_science/interface_realization.png){width=15%}


### Activity Diagrams

Activity diagrams are the more powerful version of flow charts: they represent the flux of an activity in detail, allowing to have a better understanding of a process or algorithm.

![Example of an activity diagram](./images/computer_science/activity.png){width=30%}

#### Start and End Nodes

Each diagram begins what a "start node", represented with a filled black circle, and they end with an "end node" which is represented with a filled black circle inside of a hollow circle.

![Example of activity diagrams start and end nodes](./images/computer_science/activity_start_end.png){width=30%}

#### Actions

Each action taken by the software is represented in the diagram via a rounded rectangle, a very short description of the action is written inside the rounded rectangle space.

![Example of Action in activity diagrams](./images/computer_science/activity_action.png){width=30%}

#### Decisions (Conditionals) and loops

Decisions and loops are enclosed in diamonds. If a condition needs to be written, the diamond can become an hexagon, to make space for the condition to be written or guards can be used to express the condition.

![Example of decision, using hexagons to represent the condition](./images/computer_science/activity_decision_hex.png){width=30%}

![Example of loops, using guards to represent the condition](./images/computer_science/activity_loop_guards.png){width=30%}

All the branches that depend on a condition or are part of a loop start and end on a diamond, as shown below.

![Example of how nested loops and conditions are performed](./images/computer_science/nested_activity.png){width=30%}

#### Synchronization

Synchronization (or parallel processing) is represented in activity diagrams by using filled black bars that enclose the concurrent processes: the bars are called "synchronization points" or "forks" and "joins"

![Example of concurrent processes in activity diagrams](./images/computer_science/activity_concurrent.png){width=30%}

In the previous example, the activities "Send Order Confirmation" and "Process Order" are processed in parallel, independently from each other, the first activity that finishes will wait until the other activity finishes before entering the end node.

#### Signals

Signals are used to represent how activities can be influenced or modified from outside the system. There are two symbols used to represent signals.

The "Sent Signal" symbol is represented with a convex pentagon (which reminds an arrow going away from our system), while the "Received Signal" is represented by a concave pentagon (which reminds a "slot" where the "sent signal" symbol can connect to).

![Example of signals in activity diagrams](./images/computer_science/activity_signals.png){width=30%}

\placeholder

#### Swimlanes

Swimlanes are a way to organize and group related activities in columns. For instance a shopping activity diagram can have the "Customer", "Order", "Accounting" and "Shipping" swimlanes, each of which contains activities related to their own categories.

![Example of swimlanes in activity diagrams](./images/computer_science/activity_swimlanes.png){width=50%}

#### Notes

As with Use Case and Class diagrams, Activity Diagrams can make use of notes, in the same way as the other two diagrams we presented in this book do.

![Example of a note inside of an activity diagram](./images/computer_science/activity_notes.png){width=50%}

#### A note on activity diagrams

The components of activity diagrams shown here are just a small part of the used components, but they should be enough to get you started designing and reading most of the activity diagrams that exist.

Generic Programming
--------------------

Sometimes it may be necessary (mostly in the case of containers) to have the same kind of code to work on different data types, which means that we need to **abstract types into variables** and be able to code accounting for such types.

**Generic Programming** is a blanket-term that defines a style of computer programming where algorithms are written in terms of "to be specified later" data types, this usually applies to languages that make use of *static typing*~[g]~.

Advanced Containers
-------------------

This section is dedicated to give some basic explanation of some advanced containers that are used in computer science, allowing us to make an informed choice when we want to implement some even more advanced containers in the future.

We will include big-O performance counters for the basic functions of: adding/removing and item at the beginning, adding/removing an item at the end, adding/removing an item in an arbitrary position and indexing at a certain position.

This section is in no way exhaustive, but should be enough to make an informed decision on what containers to use for our components, according to necessities.

### Dynamic Arrays

In many languages, arrays are sized statically, with a size decided at compile time. This severely limits the array's usefulness.

Dynamic Arrays are a wrapper around arrays, allowing it to extend its size when needed. This usually entails some additional operations when inserting or deleting an item.

![Dynamic Arrays Reference Image](./images/computer_science/dynamic_arrays.png){width=40%}

#### Performance Analysis

Indexing an item is immediate, since arrays allow to natively index themselves.

Inserting an item at the beginning is a heavy task, since it requires either moving all the present items or rebuilding the internal native array. Such operations require copying or moving each element, giving us a time complexity averaging on `O(n)`.

![Adding an element at the beginning of a Dynamic Array](./images/computer_science/dynamic_arrays_insert_head.png){width=90%}

Inserting an item at the end, if we keep a pointer to the last item inserted, is an operation that usually happens immediately (time complexity `O(1)`), but when the array is full, we need to instantiate a new native array (usually double the size of the current one) and copy all elements inside the new array (operation that has time complexity of `O(n)`). Since the number of `O(1)` operations outweighs by a long shot the number of `O(n)` operations, it's possible to demonstrate that in the long run appending an item at the end of a dynamic array has a time complexity averaging around `O(1)`.

![Adding an element at the end of a Dynamic Array](./images/computer_science/dynamic_arrays_insert_tail.png){width=75%}

Inserting an item in an arbitrary position, much like inserting an item at the beginning requires moving some items further into the array, potentially all of them (when the arbitrary position is the beginning of the array), thus giving us a time complexity of `O(n)`. Such operation could trigger an array resize, which has no real influence on the estimate.

![Adding an element at an arbitrary position of a Dynamic Array](./images/computer_science/dynamic_arrays_insert_arbitrary.png){width=90%}

Some implementations of the Dynamic Arrays try to save space when the number of items goes lower than $\frac{1}{4}$ of the array capacity during a deletion, the internal array is rebuilt with half the size. Such operation has a time complexity of `O(n)`.

Some other implementations use a $\frac{1}{4}$/$\frac{3}{4}$ rule, halving the array capacity when the item deletion brings the number of items lower than $\frac{1}{4}$ of the array and doubling it when an insertion makes the number of elements higher than $\frac{3}{4}$ of the array capacity.

**Note:** Not all programming languages have native support for arrays, for instance Python uses lists.

\placeholder
<!-- TODO: Code? -->

| Operation                  | Average Cost           |
| :---------:                | :-----:                |
| Indexing                   | O(1)                   |
| Insert/Delete At Beginning | O(n)                   |
| Insert/Delete At End       | O(1) amortized         |
| Insert/Delete at position  | O(n)                   |

Table: Performance table for Dynamic Arrays

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Dynamic Array

**When To Use it**          All situations that require direct indexing of a container, but insertions and removals are not extremely common, and usually take the form of "push back" (insertion at the end)

**Advantages**              Direct Indexing, Fast iteration through all the elements, given by the fact that arrays are stored compact in memory, fast appending.

**Disadvantages**           Slow insertions in arbitrary positions and at the head of the array.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Dynamic Arrays

### Linked Lists

Linked Lists are a data structure composed by "nodes", each node contains data and a reference to the next node in the linked list. Differently from arrays, nodes may not be contiguous in memory, which makes indexing problematic.

![Linked List Reference Image](./images/computer_science/linked_list_reference.png){width=60%}

Some implementations feature a pointer to the last element of the list, to make appending items at the end easier and quicker.

![Double-Ended Linked List Reference Image](./images/computer_science/de_linked_list_reference.png){width=60%}

#### Performance Analysis

Since we only have a handler on the first node, indexing requires us to scan all the elements until we reach the one that was asked for. This operation has a potential time complexity of `O(n)`.

Inserting an item at the beginning is immediate, we just need to create a new node, make it point at the current head of the list and then update our "handle" to point at the newly created node. The number of operations is independent of how many data we already have, so the time complexity is `O(1)`.

![Inserting a new node at the beginning of a linked list](./images/computer_science/linked_list_insert_head.png){width=90%}

Appending an item at the end has a time complexity that varies depending on the chosen implementation: if the list has a reference to the final node, we just need to create a new node, update the final node's reference (usually called "next") to point at the new node and then update the reference to the final node to point at the newly created node (time complexity `O(1)`). If our queue doesn't have such reference, we will need to scan the whole list to find the final node (time complexity `O(n)`).

![Inserting a new node at the end of a (double-ended) linked list](./images/computer_science/de_linked_list_insert_tail.png){width=90%}

Inserting at an arbitrary position requires us to scan the list until we find the position that we want, after that we just need to split and rebuild the references correctly, which is a fast operation.

![Inserting a new node at an arbitrary position in a (double-ended) linked list](./images/computer_science/de_linked_list_insert_arbitrary.png){width=90%}

<!-- TODO: Code? -->
\placeholder

| Operation                  | Average Cost                         |
| :---------:                | :-----:                              |
| Indexing                   | O(n)                                 |
| Insert/Delete At Beginning | O(1)                                 |
| Insert/Delete At End       | O(1) for double-ended, o(n) otherwise|
| Insert/Delete at position  | time to search + O(1)                |

Table: Performance table for Linked Lists

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Linked List

**When To Use it**          All situations that require quick insertions/removals, either on the head or the tail (stacks or queues).

**Advantages**              Very fast insertions/removals, quite fast iteration through all the elements.

**Disadvantages**           Slow indexing at an arbitrary position. Sorting can be complex.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Linked Lists

### Doubly-Linked Lists

A doubly-linked list is a variation of a linked list where each node not only has a reference to its successor, but also a reference to its predecessor. This allows for easy processing of the list in reverse, without having to create algorithms that entail a huge overhead.

All the operations of insertion, indexing and deletion are performed in a similar fashion to the classic singly-linked list we saw earlier, just with an additional pointer to account for.

![Doubly Linked List Reference Image](./images/computer_science/doubly_linked_list_reference.png){width=60%}

<!-- TODO: Code? -->

\placeholder

| Operation                  | Average Cost          |
| :---------:                | :-----:               |
| Indexing                   | O(n)                  |
| Insert/Delete At Beginning | O(1)                  |
| Insert/Delete At End       | O(1)                  |
| Insert/Delete at position  | time to search + O(1) |

Table: Performance table for Doubly-Linked Lists

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Doubly-Linked List

**When To Use it**          All situations that require quick insertions/removals, either on the head or the tail (stacks or queues) or iterating through an entire list, forwards or backwards.

**Advantages**              Very fast insertions/removals, quite fast iteration through all the elements. Possibility of easily iterating the list in reverse order.

**Disadvantages**           Slow indexing at an arbitrary position. Sorting can be complex.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Linked Lists

### Hash Tables

Hash Tables are a good way to store **unordered data** that can be referred by a "key". These structures have different names, like "maps", "dictionaries" or "hash maps".

The idea behind a hash map is having a key subject to a *hash function*~[g]~ that will decide where the item will be positioned in the internal structure.

![Hash Table Reference Image (Hash Table with Buckets)](./images/computer_science/hashtable_reference.png){width=40%}

The simplest way to implement a hash table is using an "array with buckets": an array where each cell has a reference to a linked list.

On average, finding an item requires passing the key through the hash function, such hash function will tell us where the item is in our internal structure immediately. Thus giving a time complexity of $O(1)$.

Inserting has more or less the same performance, the key gets worked through the hash function, deciding which linked list will be used to store the item.

Deletion works in the same fashion, passing the key through the hash function and then deleting the value; giving a time complexity of $O(1)$

| Operation  | Average Cost   |
| :---------:| :-----:        |
| Searching  | O(1)           |
| Insert     | O(1)           |
| Delete     | O(1)           |

Table: Performance table for Hash Tables

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Hash Table

**When To Use it**          All situations that require accessing an element by a well-defined key quickly. Building unordered data sets.

**Advantages**              Fast insertions/removals, direct indexing (in absence of hash collisions) by key.

**Disadvantages**           In case of a bad hashing function, it reverts to the performance of a linked list, cannot be ordered.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Hash Tables

### Binary Search Trees (BST)

Binary search trees, sometimes called "ordered trees" are a container that have an "order relation" between their own elements.

![Binary Search Tree Reference](./images/computer_science/BST.png){width=60%}

The order relation allows us to have a tree that is able to distinguish between "bigger" and "smaller" values, thus making search really fast at the price of a tiny slowdown in insertion and deletion.

Searching in a BST is easy, starting from the root, we check if the current node is the searched value; if it isn't we compare the current node's value with the searched value.

If the searched value is greater, we search on the right child. If it is smaller, we continue our search on the left child.

Recursively executing this algorithm will lead us to find the node, if present. Such algorithm has a $O(log(n))$ time complexity.

In a similar fashion, insertion will recursively check subtrees until the right spot of the value is found. The insertion operation has the same time complexity as searching: $O(log(n))$.

Deletion is a bit more conceptually complex, since it's necessary to maintain the ordering of the nodes. Such operation has a time complexity of $O(log(n))$.

| Operation  | Average Cost        |
| :---------:| :-----:             |
| Searching  | O(log(n))           |
| Insert     | O(log(n))           |
| Delete     | O(log(n))           |

Table: Performance table for Binary Search Trees

-------------------------   ----------------------------------------------------------------------------
**Container Name**          Binary Search Tree

**When To Use it**          Situations that require good overall performance and requires fast search times.

**Advantages**              Good insertion and removal times, searching on this structure is fast.

**Disadvantages**           Given the nature of the data structure, there is no direct indexing, nor ordering.
---------------------------------------------------------------------------------------------------------

Table: Summary Table for Binary Search Trees

### Heaps

![Heap Reference Image (Min-Heap)](./images/computer_science/heap_reference.png){width=60%}

<!-- TODO -->
\placeholder

### Red-Black Trees

<!-- TODO -->
\placeholder

### Stacks

<!-- TODO -->
\placeholder

### Queues

<!-- TODO -->
\placeholder
