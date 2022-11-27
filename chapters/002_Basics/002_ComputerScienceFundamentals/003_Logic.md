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

:::: wizardry ::::
The confines between logic operations and bitwise operations can get blurry. This section introduces bitwise operations and alternative representations of data as a way to fit more data in less space. Feel free to skim over this section.
::::::::::::::::::

So far we've seen operations that work on single binary digits, which can be seen as the numeric representation of logical statements (0 meaning "false" and 1 meaning "true"). These are logic operations.

Such operations can be applied on a bit-by-bit basis to groups of bits, that's when we talk about about "bitwise operations".

$$
\begin{array}[t]{l}
0110\ 0010\ AND \\
0101\ 1010\\ \hline
0100\ 0010
\end{array}
$$

As you can see the bitwise AND operation takes each bit of the two bytes and does an "AND" operation on each one of them.

#### Packing more information with less

Let's imagine the following situation: we have a structure that represents a tile in a maze. We want to efficiently store whether each side of a certain tile has a wall.

This can be solved by using a 4-bit positive integer and having each bit represent a side of the tile: if that bit is 1, there is a wall, 0 otherwise.

After creating a convention, we can start storing data. For instance we can have the bits representing walls starting from top, going clockwise.

This convention could be summarized as follows:

- $0001$ represents a "top wall";
- $0010$ represents a "right wall";
- $0100$ represents a "bottom wall";
- $1000$ represents a "left wall".

This means that $0110$ represents a tile having two walls: on the right and bottom side (this is the integer number 6, by the way). If we wanted to check if a certain tile has a wall, we would just need to `AND` it (bitwise) with the number that represents such wall.

If the result of such operation is not zero, the wall we searched for is in our tile. Continuing with our example, if we test for a right wall we will obtain zero:

$$
\begin{array}[t]{l}
0110\ AND \\
0001 \\ \hline
0000
\end{array}
$$

But if we test for a bottom wall, we will obtain something that is not zero:

$$
\begin{array}[t]{l}
0110\ AND \\
0100 \\ \hline
0100
\end{array}
$$

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
