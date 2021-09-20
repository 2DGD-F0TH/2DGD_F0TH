{{pagebreak}}

The Maths Behind Game Development
=================================

:::::: {.epigraph author="Albert Einstein"}
Do not worry about your difficulties in Mathematics. I can assure you mine are still greater.
::::::

This book assumes you already have some minimal knowledge of maths, including but not limited to:

- Logarithms
- Exponentials
- Roots
- Equations
- Limits
- Cartesian Coordinate system

Also we will represent derivatives with the $f'(x)$ symbol, instead of the more verbose $\frac{\partial f}{\partial x}$.

In this chapter we'll take a quick look (or if you already know them, a refresher) on the basic maths needed to make a 2D game.

Some useful symbols
-------------------

While reading this book, we may need to delve into some mathematical lingo that not everyone may understand immediately, so here's a small glossary of some of mathematical the symbols we may use.

- $x \in S$ Denotes a "set membership", so the object to the left of the symbol is an element of the set at the right: x is an element inside the set S;
- $A \subset B$ Denotes a "subset relationship": A is a subset of B;
- $A \subseteq B$ Denotes a "subset relationship" where equality is possible: A is a subset of B, but also it may happen that A equals B;
- $A \cup B$ Denotes "set union", the result is composed by all elements of A and B, combined;
- $A \cap B$ Denotes "set intersection", the result is composed by all elements of A that are also found in B;
- $\forall$ Means "for all";
- $\exists$ Means "exists";
- $\exists !$ Means "exists only one";
- $P \rightarrow Q$ Means "implies", so you can read this as "P implies Q" or "if P is true then Q is true";
- $P \leftrightarrow Q$ Logical equivalence: means "if and only if" or "is equivalent", so you can read this as "P is equivalent to Q" or "P if and only if Q";

The modulo operator
--------------------

Very basic, but sometimes overlooked, function in mathematics is the "modulo" function (or "modulo operator"). Modulo is a function that takes 2 arguments, let's call them "a" and "b", and returns the remainder of the division represented by $a / b$.

So we have examples like $mod(3,2)=1$ or $mod(4,5)=4$ and $mod(8,4)=0$.

In most programming languages the modulo function is hidden behind the operator "%", which means that the function $mod(3,2)$ is represented with $3 \% 2$.

The modulo operator is very useful when we need to loop an ever-growing value between two values (as will be shown in [infinitely scrolling backgrounds](#infiniback)).

:::: pitfall ::::
Be careful when using the modulo operator with negative arguments: it may lead to unexpected results, which may depend on the programming language you are using.
::::
