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

Vectors {#vectors}
--------

For our objective, we will simplify the complex matter that is vectors as much as possible.

In the case of 2D game development, a vector is just a pair of values `(x,y)`.

Vectors usually represent a force applied to a body, its velocity or acceleration and are graphically represented with an arrow.

![Image of a vector](./images/maths/vector.svg){width=30%}

The pain of learning about vectors is paid off by their capacity of being added and subtracted among themselves, as well as being multiplied by a number (called a "scalar") and between themselves.

### Adding and Subtracting Vectors

Adding vectors is as easy as adding its "members". Let's consider the following vectors:

$v = (2,4)$

$u = (1,5)$

The sum vector `s` will then be:

$s = u + v = (2+1, 4+5) = (3,9)$

Graphically it can be represented by placing the tail of the arrow `v` on the head of the arrow `u`, or vice-versa:

![Graphical representation of a sum of vectors](./images/maths/vector_sum.svg){width=30%}

### Scaling Vectors {#scalingvectors}

There may be situations where you need to make a vector $x$ times longer. This operation is called "scalar multiplication" and it is performed as follows:

$v = (2,4)$

$3 \cdot v = (2 \cdot 3, 4 \cdot 3) = (6,12)$

![Example of a vector multiplied by a value of 3](./images/maths/vector_mul_3.svg){width=30%}

Obviously this works with scalars with values between $0$ and $1$:

$v = (2,4)$

$\frac{1}{2} \cdot v = (\frac{1}{2} \cdot 2, \frac{1}{2} \cdot 4) = (1,2)$

![Example of a vector multiplied by a value of 0.5](./images/maths/vector_mul_half.svg){width=30%}

When you multiply the vector by a value less than $0$, the vector will rotate by $180\degree$.

$v = (2,4)$

$-2 \cdot v = (-2 \cdot 2, -2 \cdot 4) = (-4, -8)$

![Example of a vector multiplied by a value of -2](./images/maths/vector_mul_minus2.svg){width=30%}

### Dot Product

The dot product (or scalar product, projection product or inner product) is defined as follows:

Given two n-dimensional vectors $v = [v_1, v_2, ... v_n]$ and $u = [u_1, u_2, ..., u_n]$ the dot product is defined as:

$$ v \cdot u = \sum\limits_{i=1}^n (v_i \cdot u_i) = (v_1 \cdot u_1) + ... + (v_n \cdot u_n)$$

So in our case, we can easily calculate the dot product of two two-dimensional vectors $v = [v_1, v_2]$ and $u = [u_1, u_2]$ as:

$$ v \cdot u = (v_1 \cdot u_1) + (v_2 \cdot u_2)$$

Let's make an example:

Given the vectors $v = [1,2]$ and $u = [4,3]$, the dot vector is:

$$ v \cdot u = (1 \cdot 4) + (2 \cdot 3) = 4 + 6 = 10 $$

### Vector Length and Normalization

Given a vector $a = [a_1, a_2, ..., a_n]$, you can define the length of the vector as:

$$ ||a|| = \sqrt{a_1^2 + a_2^2 + ... + a_n^2}$$

Or alternatively

$$ ||a|| = \sqrt{a \cdot a}$$

We can get a 1-unit long vector by "normalizing" it, getting a vector that is useful to affect (or indicate) direction without affecting magnitude. A normalized vector is usually indicated with a "hat", so the normalized vector of $a = [a_1, a_2, ..., a_n]$ is

$$ \hat{a} = \frac{a}{||a||} $$

Knowing that the length of a vector is a scalar (a number, not a vector), normal scalar multiplication rules apply. (See [Scaling Vectors](#scalingvectors))

Geometry
--------

Among all the math we found so far (and the math we will explain later), we cannot avoid talking a bit about geometry: in this book we will talk about the minimal amount of geometry necessary to understand the underlying concepts of what's coming up.

### Convex vs Concave polygons {#conc_conv}

A polygon is considered convex essentially when **any line** (not tangent to an edge or corner) drawn through the shape crosses the shape itself only twice (at its ends).

![Example of a convex shape](./images/maths/convex_shape.svg){width=35%}

Any shape where you can find at least one line that crosses the shape more than twice is considered "non-convex" (commonly referred as "concave").

![Example of a concave shape](./images/maths/concave_shape.svg){width=35%}

::: pitfall :::
Not all non-convex shapes are technically called "concave" (they should be called "non-convex"), but for the sake of simplicity we'll use the term "non-convex" and "concave" interchangeably in this book.
:::::::::::::::

### Self-intersecting polygons

Contrary to what many think, polygons can self-intersect too, which can make calculations a lot harder.

![Example of a self-intersecting polygon](./images/maths/self_intersecting_poly.svg){width=40%}

For the sake of game development, we will usually talk about simple polygons which are polygons that don't self-intersect and have no holes in them. More strictly we will (for 99.9% of the time) talk about **convex simple polygons**.

### Straight Lines and their equations

One of the main topics we will encounter over and over in our game development adventure will be "straight lines". We will need to draw them, see if two straight lines collide, project stuff onto them, and much more. So it's important that we know them well.

Here's a straight line:

$$ ax + by + c = 0$$

That's not what you expected, right? What you've seen is the "general form" of a straight line's equation, because you can represent lines using equations (also circles, and other stuff). This is not a much-used form, though, probably the most used form is called the "slope-intercept form":

$$ y = mx + q$$

::: trivia :::
To transform a "general form" equation into the relative "slope-intercept from" just remember the following formulas:

$$ m = -\frac{a}{b}\ \ q = -\frac{c}{b}$$

This doesn't work well when $b=0$, which will be subject of the next "pitfall".
:::::::::::::

Where in this case $m$ is the *slope* of our straight line, and $q$ represents the so-called *y-intercept* (the value of $y$ when $x=0$). If $q=0$ the line goes through the origin of the Cartesian coordinate system, if $m=0$ the line is horizontal.

::: pitfall :::
"Vertical straight lines" is where the slope-intercept form fails, in fact vertical straight lines have an equation in the form of $x=k$, which would mean that $b=0$ which is problematic (see previous trivia).
:::::::::::::::

#### Getting the equation of a straight line, given two points

We all know that given two points we can strike one and only one line. How many times did you measure two points (maybe while doing some DIY) and stroke a line between them?

It will be useful in our adventure to be able to get the equation of a straight line starting from two points, so let's call our two points $P(x_1,y_1)$ and $Q(x_2,y_2)$, then the straight line that crosses both those points will have equation:

$$ \frac{y-y_1}{x-x_1} = \frac{y_2 - y_1}{x_2 - x_1}$$

This may seem really complicated, but with some small calculations we can reach a formula for our straight line in any form (generic or "slope-intercept").

::: pitfall :::
Again, this formula fails when we are dealing with "vertical lines", because the denominator at the right side of the equation will be zero. But in that case we'll already know the formula: it will be $x=x_1$ (which in turn will be equal to $x_2$)
:::::::::::::::

#### Getting the equation, given the slope and a point

If we have a point $P(x_p, y_p)$ and the slope $m$ (for instance if we need to find a line perpendicular to another line), in that case we can use the following formula:

$$ y - y_p = m(x - x_p)$$

::: pitfall :::
Guess what? This (again) doesn't allow us to create "vertical lines", because we need a slope value, which we don't have when it comes to vertical lines. You can see (non rigorously) a vertical line as a line with "infinite slope".
:::::::::::::::

### Projections {#projections}

In some situations (as you will see in the [SAT Theorem](#SAT)), we may need to get to project polygons onto a line, this usually involves projecting **points** to a line.

Given the formulas we've seen earlier, and doing some thinking, we can easily project a point onto any straight line. Let's see how to do it.

We will assume that we have a point $P(x_p,y_p)$ that we want to project onto a line $r$ with equation $y=mx+q$, with $m \neq 0$ (thus excluding horizontal lines). We will call the projected point "P onto r" with the name $P_r(x_r,y_r)$.

![Projecting the point P onto the line r](./images/maths/projection.svg){width=40%}

First, we need to find the line that goes through $P$ and is perpendicular to $r$, this is really easy. To find a slope $m_1$ of a line perpendicular to another line with slope $m$ we use the formula

$$m_1 = \frac{1}{m}$$

::: pitfall :::
This is why we excluded the case $m=0$ (horizontal lines), if we didn't we would have the chance of having $m_1 = \frac{1}{0}$ which doesn't make sense
:::::::::::::::

Now we have a point and a slope, so we can use one of the formulas we've already seen to find the line with that slope that crosses $P$:

$$y - y_p = m_1(x - x_p) \Leftrightarrow y - y_p = \frac{1}{m}(x - x_p)$$

To find $P_r$ we just need to find the point where the two lines collide, which is the solution to the equation system:

$$
\begin{cases}
y = mx + q\\
y - y_p = \frac{1}{m}(x - x_p)
\end{cases}
$$

Which finds solution in:

$$
\begin{cases}
x = \frac{x_p + mq}{1-m^2}\\
y = \frac{mx_p + q}{1-m^2}
\end{cases}
$$

The coordinates $x$ and $y$ we just found are actually the coordinates $x_r$ and $y_r$ of our projected point $P_r$.

Matrices
---------

### What is a matrix

Matrices are essentially an $m \times n$ array of numbers, which are used to represent linear transformations.

Here is an example of a $2 \times 3$ matrix.

$$A_{2,3} =\begin{bmatrix}
    2 & 1 & 4\\
    3 & 2 & 0
\end{bmatrix}$$

### Matrix sum and subtraction

Summing and subtracting $m \times n$ matrices is done by summing or subtracting each element, here is a simple example. Given the following matrices:

$$ A_{2,3} = \begin{bmatrix}
    2 & 1 & 4\\
    3 & 2 & 0
\end{bmatrix} B_{2,3} = \begin{bmatrix}
    1 & 3 & 0\\
    4 & 2 & 4
\end{bmatrix}$$

We have that:

$$ A_{2,3} + B_{2,3} = \begin{bmatrix}
    2 & 1 & 4\\
    3 & 2 & 0
\end{bmatrix} + \begin{bmatrix}
    1 & 3 & 0\\
    4 & 2 & 4
\end{bmatrix} = \begin{bmatrix}
    2+1 & 1+3 & 4+0 \\
    3+4 & 2+2 & 0+4
\end{bmatrix} = \begin{bmatrix}
    3 & 4 & 4\\
    7 & 4 & 4
\end{bmatrix}$$

### Multiplication by a scalar

Multiplication by a scalar works in a similar fashion to vectors, given the matrix:

$$A_{2,3} =\begin{bmatrix}
    2 & 1 & 4\\
    3 & 2 & 0
\end{bmatrix}$$

Multiplication by a scalar is performed by multiplying each member of the matrix by the scalar, like the following example:

$$3 \cdot A_{2,3} = 3 \cdot \begin{bmatrix}
    2 & 1 & 4\\
    3 & 2 & 0
\end{bmatrix} = \begin{bmatrix}
    3 \cdot 2 & 3 \cdot 1 & 3 \cdot 4\\
    3 \cdot 3 & 3 \cdot 2 & 3 \cdot 0
\end{bmatrix} = \begin{bmatrix}
    6 & 3 & 12\\
    9 & 6 & 0
\end{bmatrix}
$$

### Transposition

Given an $m \times n$ matrix $A$, its transposition is an $n \times m$ matrix $A^T$ constructed by turning rows into columns and columns into rows.

Given the matrix:

$$A_{2,3} =\begin{bmatrix}
    2 & 1 & 4\\
    3 & 2 & 0
\end{bmatrix}$$

The transpose matrix is:

$$A_{2,3}^T  = \begin{bmatrix}
    2 & 3\\
    1 & 2\\
    4 & 0
\end{bmatrix}$$

### Multiplication between matrices

Given 2 matrices with sizes $m \times n$ and $n \times p$ (mind how the number of rows of the first matrix is the same of the number of columns of the second matrix):

$$ A_{3, 2} = \begin{bmatrix}
    2 & 3\\
    1 & 2\\
    4 & 0
\end{bmatrix} B_{2,3} = \begin{bmatrix}
    2 & 3 & 4\\
    0 & 1 & 0
\end{bmatrix}$$

We can calculate the multiplication between these two matrices, in the following way.

First of all let's get the size of the resulting matrix, which will be always $m \times p$.

Now we have the following situation:

$$ \begin{bmatrix}
    2 & 3\\
    1 & 2\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    2 & 3 & 4\\
    0 & 1 & 0
\end{bmatrix} = \begin{bmatrix}
    ? & ? & ?\\
    ? & ? & ?\\
    ? & ? & ?
\end{bmatrix}$$

Matrix multiplication is called a "rows by columns" multiplication, so to calculate the first row - first column value we'll need the first row of one matrix and the first column of the other.

$$ \begin{bmatrix}
    \textcolor{red}{2} & \textcolor{red}{3}\\
    1 & 2\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    \textcolor{red}{2} & 3 & 4\\
    \textcolor{red}{0} & 1 & 0
\end{bmatrix} = \begin{bmatrix}
    \textcolor{red}{?} & ? & ?\\
    ? & ? & ?\\
    ? & ? & ?
\end{bmatrix}$$

The values in the example will be combines as follows:

$$2 \cdot 2 + 3 \cdot 0 = 4$$

Obtaining the following:

$$ \begin{bmatrix}
    2 & 3\\
    1 & 2\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    2 & 3 & 4\\
    0 & 1 & 0
\end{bmatrix} = \begin{bmatrix}
    4 & ? & ?\\
    ? & ? & ?\\
    ? & ? & ?
\end{bmatrix}$$

Let's try the next value:

$$ \begin{bmatrix}
    \textcolor{red}{2} & \textcolor{red}{3}\\
    1 & 2\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    2 & \textcolor{red}{3} & 4\\
    0 & \textcolor{red}{1} & 0
\end{bmatrix} = \begin{bmatrix}
    4 & \textcolor{red}{?} & ?\\
    ? & ? & ?\\
    ? & ? & ?
\end{bmatrix}$$

The values will be combined as follows:

$$ 2 \cdot 3 + 3 \cdot 1 = 9$$

Obtaining:

$$ \begin{bmatrix}
    2 & 3\\
    1 & 2\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    2 & 3 & 4\\
    0 & 1 & 0
\end{bmatrix} = \begin{bmatrix}
    4 & 9 & ?\\
    ? & ? & ?\\
    ? & ? & ?
\end{bmatrix}$$

Same goes for the last value, when we are done with the first row, we keep going similarly with the second row:

$$ \begin{bmatrix}
    2 & 3\\
    \textcolor{red}{1} & \textcolor{red}{2}\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    \textcolor{red}{2} & 3 & 4\\
    \textcolor{red}{0} & 1 & 0
\end{bmatrix} = \begin{bmatrix}
    4 & 9 & 8\\
    \textcolor{red}{?} & ? & ?\\
    ? & ? & ?
\end{bmatrix}$$

Which leads to the following calculation:

$$ 1 \cdot 2 + 2 \cdot 0 = 2$$

Which we will insert in the result matrix:

$$ \begin{bmatrix}
    2 & 3\\
    1 & 2\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    2 & 3 & 4\\
    0 & 1 & 0
\end{bmatrix} = \begin{bmatrix}
    4 & 9 & 8\\
    2 & ? & ?\\
    ? & ? & ?
\end{bmatrix}$$

You can try completing this calculation yourself, the final result is as follows:

$$ \begin{bmatrix}
    2 & 3\\
    1 & 2\\
    4 & 0
\end{bmatrix} \times \begin{bmatrix}
    2 & 3 & 4\\
    0 & 1 & 0
\end{bmatrix} = \begin{bmatrix}
    4 & 9 & 8\\
    2 & 5 & 4\\
    8 & 12 & 16
\end{bmatrix}$$

Multiplication between matrices is **non commutative**, which means that the result of $A \times B$ is not equal to the result of $B \times A$, actually one of the results may not even be possible to calculate.

### Other uses for matrices

Matrices can be used to quickly represent equation systems, with equation that depend on each other. For instance:

$$ \begin{bmatrix}
    2 && 3 && 6\\
    1 && 4 && 9
   \end{bmatrix} \begin{bmatrix}
   x\\
   y\\
   z
   \end{bmatrix} = \begin{bmatrix}
   4\\
   5
\end{bmatrix}$$

Can be used to represent the following system of equations:

$$
\begin{cases}
    2x + 3y + 6z = 4\\
    1x + 4y + 9z = 5
\end{cases}
$$

Or, as we'll see, matrices can be used to represent transformations in the world of game development.

Trigonometry
------------

When you want to develop a game, you will probably find yourself needing to rotate items relative to a certain point or relative to each other. To do so, you need to know a bit of trigonometry, so here we go!

### Radians vs Degrees

In everyday life, angles are measured in degrees, from 0 to 360 degrees. In some situations in math, it is more comfortable to measure angles using radians, from 0 to $2 \pi$.

You can convert back and forth between radians and degrees with the following formulas:

$$angle\ in\ degrees = angle\ in\ radians \cdot \frac{180}{\pi}$$

$$angle\ in\ radians = angle\ in\ degrees \cdot \frac{\pi}{180}$$

This book will always refer to angles in radians, so here are some useful conversions, ready for use:

| Degrees   | Radians         |
| :-------: | :-------:       |
| 0°        | 0               |
| 30°       | $\frac{\pi}{6}$ |
| 45°       | $\frac{\pi}{4}$ |
| 60°       | $\frac{\pi}{3}$ |
| 90°       | $\frac{\pi}{2}$ |
| 180°      | $\pi$           |
| 360°      | $2 \pi$         |

Table: Conversion between degrees and Radians

### Sine, Cosine and Tangent

The most important trigonometric functions are sine and cosine. They are usually defined in reference to a "unit circle" (a circle with radius 1).

Given the unit circle, let a line through the origin with an angle $\theta$ with the positive side of the x-axis intersect such unit circle. The x coordinate of the intersection point is defined by the measure $cos(\theta)$, while the y coordinate is defined by the measure $sin(\theta)$.

![Unit Circle definition of sine and cosine](./images/maths/sincos.png){width=40%}

For the purposes of this book, we will just avoid the complete definition of the tangent function, and just leave it as a formula of sine and cosine:

$$tan(\theta) = \frac{sin(\theta)}{cos(\theta)} $$

### Pythagorean Trigonometric Identity

One of the most important identities in Trigonometry is the "Pythagorean Trigonometric Identity", which is expressed as follows, valid for each angle $\theta$:

$$ sin^2(\theta) + cos^2(\theta) = 1$$

Using this identity, you can express functions in different ways:

$$ cos^2(\theta) = 1 - sin^2(\theta) $$

$$ sin^2(\theta) = 1 - cos^2(\theta) $$

Also remember that $sin^2(\theta) = (sin(\theta))^2$ and $cos^2(\theta) = (cos(\theta))^2$.

### Reflections

Sometimes we may need to reflect an angle to express it in an easier way, and their trigonometric formulas will be affected, so the following formulas may come of use:

| Reflection Formulas                                  |
| :-------------------:                                |
| $sin(-\theta) = - sin(\theta)$                       |
| $cos(-\theta) = cos(\theta)$                         |
| $sin(\frac{\pi}{2} - \theta) = cos(\theta)$          |
| $cos(\frac{\pi}{2} - \theta) = sin(\theta)$          |
| $sin(\pi - \theta) = sin(\theta)$                    |
| $cos(\pi - \theta) = - cos(\theta)$                  |
| $sin(2\pi - \theta) = - sin(\theta) = sin(- \theta)$ |
| $cos(2\pi - \theta) = cos(\theta) = cos(- \theta)$   |

Table: Some reflection formulas for trigonometry

### Shifts

Trigonometric functions are periodic, so you may have an easier time calculating them when their arguments are shifted by a certain amount. Here we can see some of the shift formulas:

| Shift Formulas                                    |
| :------------:                                    |
| $sin(\theta \pm \frac{\pi}{2}) = \pm cos(\theta)$ |
| $cos(\theta \pm \frac{\pi}{2}) = \mp sin(\theta)$ |
| $sin(\theta + \pi) = - sin(\theta)$               |
| $cos(\theta + \pi) = - cos(\theta)$               |
| $sin(\theta + k \cdot 2\pi) = sin(\theta)$        |
| $cos(\theta + k \cdot 2\pi) = cos(\theta)$        |

Table: Some Shift Formulas for Trigonometry

### Trigonometric Addition and subtraction

Sometimes you may need to express a trigonometric formula with a complex argument by splitting such argument into different trigonometric formulas. If such argument is a sum or subtraction of angles, you can use the following formulas:

| Addition/Difference Identities                                            |
| :----------------------------:                                            |
| $sin(\alpha \pm \beta) = sin(\alpha)cos(\beta) \pm cos(\alpha)sin(\beta)$ |
| $cos(\alpha \pm \beta) = cos(\alpha)cos(\beta) \mp sin(\alpha)sin(\beta)$ |

Table: Some addition and difference identities in trigonometry

### Double-Angle Formulae

Other times (mostly on paper) you may have an argument that is a multiple of a known angle, in that case you can use double-angle formulae to calculate them.

| Double-Angle Formulae                          |
| :-------------------:                          |
| $sin(2\theta) = 2sin(\theta)cos(\theta)$       |
| $cos(2\theta) = cos^2(\theta) - sin^2(\theta)$ |

Table: Some double-angle formulae used in trigonometry

### Inverse Formulas

As with practically all math formulas, there are inverse formulas for sine and cosine, called $arcsin$ and $arccos$, which allow to find an angle, given its sine and cosine.

In this book we won't specify more, besides what could be the most useful: the 2-argument arctangent.

This formula allows you to find the angle of a vector, relative to the coordinate system, given the `x` and `y` coordinates of its "tip", such angle $\theta$ is defined as:

$$ \theta = arctan(\frac{y}{x}) $$

![Graphical plotting of the angle of a vector](./images/maths/arctan2.png){width=40%}

Numerical Analysis
------------------

Here we will give some pointers over some algorithms and methods that may be useful to better explain some topics treated in this book. Feel free to skip or quickly read this section if you don't want to dive into too much detail over this kind of math.

### Newton-Raphson method {#newtonmethod}

Also known as Newton's method, this is an iterative algorithm that is used to get progressively better approximations to the roots of a function.

The algorithm starts with a "guess", called $x_0$, and produces the first approximation using the formula:

$$ x_1 = x_0 - \frac{f(x_0)}{f'(x_0)}$$

Each subsequent guess (and thus iteration) can be obtained similarly by using the formula:

$$ x_{n+1} = n - \frac{f(n)}{f'(n)}$$

And such guess will be more precise than the previous one (if we don't consider some situations where approaching the root can be problematic or not possible). The algorithm will stop when you reach an approximation that is "good enough".

Obviously all limitations of standard functions apply, such as domain and trouble with divisions by zero.

Coordinate Systems on computers
---------------------------------

When it comes to 2D graphics on computers, our world gets quite literally turned upside down.

In our math courses we learned about the Coordinate Plane, with an origin and an `x` axis going from left to right and a `y` axis going from bottom to top, where said axis cross it's called the "Origin".

![Image of a coordinate plane](images/maths/coord.svg){width=30%}

When it comes to 2D graphics on computers and game development, the coordinate plane looks like this:

![Image of a screen coordinate plane](images/maths/screen_coord.svg){width=30%}

The origin is placed on the top left of the screen (at coordinates `(0,0)`) and the `y` axis is going from top to bottom. It's a little weird at the beginning, but it's not hard to get used to it.

Transformation Matrices
-----------------------

There will be a time, in our game development journey where we need to rotate an object, and that's bound to be pretty easy because rotation is something that practically all engines and toolkits do natively. But also there will be times where we need to do transformations by hand.

An instance where it may happen is rotating an item relative to a certain point or another item: imagine a squadron of war planes flying in formation, where all the planes will move (and thus rotate) relative to the "team leader".

In this chapter we'll talk about the 3 most used transformations:

- Stretching/Squeezing/Scaling;
- Rotation;
- Shearing.

### Stretching

Stretching is a transformation that enlarges all distances in a certain direction by a defined constant factor. In 2D graphics you can stretch (or squeeze) along the x-axis, the y-axis or both.

If you want to stretch something along the x-axis by a factor of $k$, you will have the following system of equations:

$$
\begin{cases}
    x' = k \cdot x\\
    y' = y
\end{cases}
$$

which is translated in the following matrix form:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = \begin{bmatrix}
    k & 0\\
    0 & 1
\end{bmatrix} \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$

Likewise, you can stretch something along the y-axis by a factor of $k$ by using the following matrices:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = \begin{bmatrix}
    1 & 0\\
    0 & k
\end{bmatrix} \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$

You can mix and match the factors and obtain different kinds of stretching, if the same factor $k$ is used both on the x and y-axis, we are performing a *scaling* operation.

In instead of stretching you want to squeeze something by a factor of $k$, you just need to use the following matrices for the x-axis:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = \begin{bmatrix}
    \frac{1}{k} & 0\\
    0 & 1
\end{bmatrix} \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$

and respectively, the y-axis:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = \begin{bmatrix}
    1 & 0\\
    0 & \frac{1}{k}
\end{bmatrix} \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$

### Rotation

If you want to rotate an object by a certain angle $\theta$, you need to decide upon two things (besides the angle of rotation):

- Direction of rotation (clockwise or counterclockwise);
- The point of reference for the rotation.

#### Choosing the direction of the rotation

We will call $T_R$ the transformation matrix for the "rotation" functionality.

Similarly to stretching, rotating something of a certain angle $\theta$ leads to the following matrix form:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = T_R \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$

If we want to rotate something **clockwise**, relative to its reference point, we will have the following transformation matrix:

$$
T_R = \begin{bmatrix}
    cos(\theta) & sin(\theta)\\
    -sin(\theta) & cos(\theta)
\end{bmatrix}
$$

If instead we want our rotation to be **counterclockwise**, we will instead use the following matrix:

$$
T_R = \begin{bmatrix}
    cos(\theta) & -sin(\theta)\\
    sin(\theta) & cos(\theta)
\end{bmatrix}
$$

:::: pitfall ::::
These formulas **assume that the x-axis points right and the y-axis points up**, if the y-axis points down in your implementation, you need to swap the matrices.
::::

#### Rotating referred to an arbitrary point

The biggest problem in rotation is rotating an object relative to a certain point: you need to know the origin of the coordinate system as well and modify the matrices as follows:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = T_R \begin{bmatrix}
    x - x_{origin}\\
    y - y_{origin}
\end{bmatrix} + \begin{bmatrix}
    x_{origin}\\
    y_{origin}
\end{bmatrix}
$$

In short, you need to rotate the item by first "bringing it centered to the origin", rotating it and then bring it back into its original position.

### Shearing

During stretching, we used the elements that are in the "main diagonal" to stretch our objects. If we modify the elements in the "antidiagonal", we will obtain shear mapping (or shearing).

Shearing will move points along a certain axis with a "strength" defined by the distance along the other axis: if we shear a rectangle, we will obtain a parallelogram.

A shear parallel to the x-axis will have the following matrix:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = \begin{bmatrix}
    1 & k\\
    0 & 1
\end{bmatrix} \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$

While a shear parallel to the y-axis will instead have the following matrix:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = \begin{bmatrix}
    1 & 0\\
    k & 1
\end{bmatrix} \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$
