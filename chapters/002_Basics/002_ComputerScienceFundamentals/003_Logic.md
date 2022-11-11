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
