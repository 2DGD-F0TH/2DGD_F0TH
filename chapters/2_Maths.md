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

### Scaling Vectors {#scalingvectors}

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

### Vector Length and Normalization

Given a vector $a = [a_1, a_2, ..., a_n]$, you can define the length of the vector as:

$$ ||a|| = \sqrt{a_1^2 + a_2^2 + ... + a_n^2}$$

Or alternatively

$$ ||a|| = \sqrt{a \cdot a}$$

We can get a 1-unit long vector by "normalizing" it, getting a vector that is useful to affect (or indicate) direction without affecting magnitude. A normalized vector is usually indicated with a "hat", so the normalized vector of $a = [a_1, a_2, ..., a_n]$ is

$$ \hat{a} = \frac{a}{||a||} $$

Knowing that the length of a vector is a scalar (a number, not a vector), normal scalar multiplication rules apply. (See [Scaling Vectors](#scalingvectors))

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

Given 2 matrices with sizes $m \times n$ and $n \times p$:

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

\placeholder

<!-- TODO: Finish -->

Coordinate Systems on computers
---------------------------------

When it comes to 2D graphics on computers, our world gets quite literally turned upside down.

In our math courses we learned about the Coordinate Plane, with an origin and an `x` axis going from left to right and a `y` axis going from bottom to top, where said axis cross it's called the "Origin".

![Image of a coordinate plane](images/maths/coord.pdf){width=30%}

When it comes to 2D graphics on computers and game development, the coordinate plane looks like this:

![Image of a screen coordinate plane](images/maths/screen_coord.pdf){width=30%}

The origin is placed on the top left of the screen (at coordinates `(0,0)`) and the `y` axis is going from top to bottom. It's a little weird at the beginning, but it's not hard to get used to it.
