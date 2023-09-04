{{pagebreak}}

The Maths Behind Game Development
=================================

:::::: {.epigraph author="Albert Einstein"}
Do not worry about your difficulties in Mathematics. I can assure you mine are still greater.
::::::

This book assumes you already have some knowledge of maths, but we will also try to keep the bar of entry as low as possible.

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

Powers and Roots
----------------

We start our revision of maths by remembering powers and roots. A power is just a short way to multiply a number by itself a certain amount of times.

For example: $2^3 = 2 \cdot 2 \cdot 2 = 8$, 2 is multiplied by itself 3 times, giving 8 as a result.

Some other examples can be $4^4 = 4 \cdot 4 \cdot 4 \cdot 4 = 256$ and $0^{9532} = 0 \cdot 0 \cdot ... \cdot 0 = 0$

One rule that we need to remember is that any number, when elevated to the zero-th power is always $1$, so $256^0=1$ as well as $2^0=1$.

:::: note ::::
Technically $0^0$ might be considered "undefined", but in most non-rigorous mathematical environments $0^0=1$ is accepted.
::::::::::::::

Roots are the inverse operation of powers, which means that if $4^2 = 16$ then $\sqrt[2]{16} = 4$

So we can say that

> The nth root of a number x is a number r so that $r^n = x$

Taking the examples of earlier, we have that $\sqrt[3]{8} = 2$, $\sqrt[4]{4} = 4$ and $\sqrt[9532]{0} = 0$. Omitting the index $n$ on the root is a short way to write the "square root", which is the root with index 2. That means:

$\sqrt[2]{4} = \sqrt{4} = 2$

:::: pitfall ::::
When talking "real numbers", there is no $\sqrt{-1}$: that would fall into the "complex numbers" category, which are a matter outside the scope of this revision. That's because there is no real number that multiplied by itself an even amount of times that would give a negative number.
To make things more complex, roots with odd indices of negative numbers are part of the real numbers instead: $\sqrt[3]{-8}=-2$ because $(-2)^3=-8$
:::::::::::::::::

Equations
---------

Equations are a way to express equality between two expressions, we've seen equations all our lives, just "hidden". Every operation is an equation.

In their more known form, equations can have one or more "unknowns", usually represented with letters (the most used are, in order $x$, $y$ and $z$) and "solving an equation" means finding values for the unknowns that make the equation true.

Here's a simple equation:

$$ 2 \cdot x = 10$$

Which can be read as "x is the number, that multiplied by 2, gives 10", the solution of this equation is $x=5$, because $2 \cdot 5 = 10$.

There are some basic rules, here's a quick rundown.

### You can add or subtract any number on both sides

This is one of the rules that will help us making things a bit easier. Let's take the following example:

$$15 + 2x = 45$$

We can subtract 15 on both sides to make our life easier:

$$- 15 + 15 + 2x = 45 - 15$$
$$2x = 30$$

### You can multiple or divide any non-zero number on both sides

This is another one of those rules that makes things a lot easier, taking the previous example:

$$2x = 30$$

We can divide each side by 2 (or multiply it by $\frac{1}{2}$) to get to the final result:

$$\frac{1}{2} \cdot 2x = 30 \cdot \frac{1}{2}$$
$$\frac{2x}{2} = \frac{30}{2}$$
$$x = 15$$

Exponentiations and Logarithms
------------------------------

Similarly to powers involving simple numbers, we can involve letters in powers too, making them "exponentiations".

$$ 2^x = 32 $$

In this case x is the number that makes the previous equation true (by the way, the result is $x=5$).

Its inverse is called a "logarithm", and it's represented as follows:

$$ \log_{2} 32 = x $$

In the previous example "2" is called the "base" of the logarithm. So this formula is read as "x is the base 2 logarithm of 32" (the result is still 5, by the way).

Here is a quick table of rules that can be used to make logarithms easier to calculate.

: Some rules that would help us calculating logarithms

+----------+-------------------------------------------+
| Rule     | Formula                                   |
+:========:+:=========================================:+
| Product  | $\log_b(x \cdot y) = \log_bx + \log_by$   |
+----------+-------------------------------------------+
| Quotient | $\log_b(\frac{x}{y}) = \log_bx - \log_by$ |
+----------+-------------------------------------------+
| Power    | $\log_b(x^p) = p \cdot \log_bx$           |
+----------+-------------------------------------------+
| Root     | $\log_b(\sqrt[p]{x}) = \frac{log_bx}{p}$  |
+----------+-------------------------------------------+

Limits
------

:::: wizardry ::::
We're entering some complex math territory here, so I will give you an "intuitive" definition of a limit. Having an idea of what it is will suffice for the needs of this book
::::::::::::::

Limits are an interesting beast: we can see them as the value a function approaches as the input approaches some value. Limits are written as follows:

$$ \lim_{x\to a} f(x) = y$$

In this case it can be read as "y is the limit, for x approaching a, of $f(x)$". Limits can be seen as, "the more x gets closer to a, the more $f(x)$ gets closer to y".

$y$ and $a$ can be any value, including infinity. In fact the following statement is true:

$$ \lim_{x\to +\infty} x = +\infty $$

The further we count down the line of numbers the closer we get to infinity. Which also means that:

$$ \lim_{x\to +\infty} \frac{1}{x} = 0 $$

Because as we are counting up with x, we are dividing 1 by bigger and bigger numbers until (at the limit) we reach 0.

:::: pitfall ::::
There are some situations where a limit cannot be determined immediately (or sometimes at all). Some of these are $+ \infty - \infty$, $0 \cdot \infty$, $\frac{\infty}{\infty}$, $\frac{0}{0}$, $\infty^0$, $0^0$ and $1^{\infty}$.
:::::::::::::::::

Derivatives
-----------

:::: note ::::
This is not a complete guide to derivatives, there is so much more to it than written in here. This is mostly for informational purposes when the term "derivative" will be used in this book.
::::::::::::::

Derivatives are technically just a limit, to be precise they are the following limit:

$$ \lim_{h\to0}\frac{f(x+h) - f(h)}{h} $$

They also have a nifty property that is used extensively in calculus: if $f'(x) > 0$ then $f(x)$ is increasing, while if $f'(x) < 0$ then $f(x)$ is decreasing. This means that the equation $f'(x)=0$ can be used to find local extrema: also known as "local minimums" and "local maximums".

There are some rules to quickly derivative functions, here we list some of the most basic.

: Some simple derivation rules (k is any constant number and e is Euler's number)

+----------+-------------------+
| Function | Derivative        |
+:========:+:=================:+
| $k$      | $0$               |
+----------+-------------------+
| $x^k$    | $k \cdot x^{k-1}$ |
+----------+-------------------+
| $e^x$    | $e^x$             |
+----------+-------------------+

Then there are rules for sums, multiplications and divisions.

: Some derivation rules for combined functions (a and b are constants)

+---------------------+------------------------------------------+
| Functions           | Derivative                               |
+:===================:+:========================================:+
| $af(x)+bg(x)$       | $af'(x) + bg'(x)$                        |
+---------------------+------------------------------------------+
| $f(x)g(x)$          | $f'(x)g(x) + f(x)g'(x)$                  |
+---------------------+------------------------------------------+
| $\frac{f(x)}{g(x)}$ | $\frac{f'(x)g(x) - f(x)g'(x)}{(g(x))^2}$ |
+---------------------+------------------------------------------+
| $f(g(x))$           | $f'(g(x)) \cdot g'(x)$                   |
+---------------------+------------------------------------------+

The Cartesian Plane
-------------------

The Cartesian plane is a plane that features a 2-dimensional coordinate system. This way we can represent points with a pair of coordinates $(x, y)$.

![Example of a Cartesian plane](./images/maths/coord.svg){width=60%}

:::: longdesc ::::
A Cartesian plane, representing a grid overlaid by two axes, 90 degrees from each other. The X axis is represented with a double arrow going left to right. The Y axis is represented with a double arrow going bottom to top. The two axes cross each other in the center of the image (at the origin) and divide the plane in 4 quadrants which are numbered I, II, III, IV, starting from top right and going counterclockwise. Each axis is numbered from -8 to +8.
::::::::::::::::::

Using a Cartesian plane we can represent the position of items, as well as their shape, space occupation, as well as vectors that represent forces, velocity and direction.

It is an essential tool for 2D game development, and it will be one of the abstractions we will use to represent items in a 2-Dimensional plane.
