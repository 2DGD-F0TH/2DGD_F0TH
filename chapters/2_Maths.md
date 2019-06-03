\null\clearpage
The Maths Behind Game Development
=================================

This book assumes you already have some minimal knowledge of maths, including but not limited to:

- Logarithms
- Exponentials
- Roots
- Equations
- Limits

In this chapter we'll take a quick look (or if you already know them, a refresher) on the basic maths needed to make a 2D game.

Vectors
--------

For our objective, we will simplify the complex matter that is vectors as much as possible.

In the case of 2D game development, a vector is just a pair of values `(x,y)`.

Vectors usually represent a force applied to a body, its velocity or acceleration and are graphically represented with an arrow.

![Image of a vector](./images/maths/vector.pdf){width=30%}

The pain of learning about vectors is paid off by their capacity of being added and subtracted among themselves, as well as being multiplied by a number (called a "scalar").

### Adding and Subtracting Vectors

Adding vectors is as easy as adding its "members". Let's consider the following vectors:

$v = (2,4)$

$u = (1,5)$

The sum vector `s` will then be:

$s = u + v = (2+1, 4+5) = (3,9)$

Graphically it can be represented by placing the tail of the arrow `v` on the head of the arrow `u`, or vice-versa:

![Graphical representation of a sum of vectors](./images/maths/vector_sum.pdf){width=30%}

### Scaling Vectors

There may be situations where you need to make a vector $x$ times longer. This operation is called "scalar multiplication" and it is performed as follows:

$v = (2,4)$

$3 \cdot v = (2 \cdot 3, 4 \cdot 3) = (6,12)$

![Example of a vector multiplied by a value of 3](./images/maths/vector_mul_3.pdf){width=30%}

Obviously this works with scalars with values between $0$ and $1$:

$v = (2,4)$

$\frac{1}{2} \cdot v = (\frac{1}{2} \cdot 2, \frac{1}{2} \cdot 4) = (1,2)$

![Example of a vector multiplied by a value of 0.5](./images/maths/vector_mul_half.pdf){width=30%}

When you multiply the vector by a value less than $0$, the vector will rotate by $180\degree$.

$v = (2,4)$

$-2 \cdot v = (-2 \cdot 2, -2 \cdot 4) = (-4, -8)$

![Example of a vector multiplied by a value of -2](./images/maths/vector_mul_minus2.pdf){width=30%}

### Dot Product

The dot product (or scalar product, projection product or inner product) is defined as follows:

Given two n-dimensional vectors $v = [v_1, v_2, ... v_n]$ and $u = [u_1, u_2, ..., u_n]$ the dot product is defined as:

$$ v \cdot u = \sum\limits_{i=1}^n (v_i \cdot u_i) = (v_1 \cdot u_1) + ... + (v_n \cdot u_n)$$

So in our case, we can easily calculate the dot product of two two-dimensional vectors $v = [v_1, v_2]$ and $u = [u_1, u_2]$ as:

$$ v \cdot u = (v_1 \cdot u_1) + (v_2 \cdot u_2)$$

Let's make an example:

Given the vectors $v = [1,2]$ and $u = [4,3]$, the dot vector is:

$$ v \cdot u = (1 \cdot 4) + (2 \cdot 3) = 4 + 6 = 10 $$

### Normalizing Vectors

<!-- TODO: Talk about how to get a vector's length/magnitude and how to obtain a unit vector from that -->

Matrices
---------

<!-- TODO: Explain matrices -->

### What is a matrix

### Matrix sum and subtraction

### Multiplication by a scalar

### Multiplication between matrices

Coordinate Systems on computers
---------------------------------

When it comes to 2D graphics on computers, our world gets quite literally turned upside down.

In our math courses we learned about the Coordinate Plane, with an origin and an `x` axis going from left to right and a `y` axis going from bottom to top, where said axis cross it's called the "Origin".

![Image of a coordinate plane](images/maths/coord.pdf){width=30%}

When it comes to 2D graphics on computers and game development, the coordinate plane looks like this:

![Image of a screen coordinate plane](images/maths/screen_coord.pdf){width=30%}

The origin is placed on the top left of the screen (at coordinates `(0,0)`) and the `y` axis is going from top to bottom. It's a little weird at the beginning, but it's not hard to get used to it.
