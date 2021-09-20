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

:::: note ::::
Multiplication between matrices is **non commutative**, which means that the result of $A \times B$ is not equal to the result of $B \times A$: actually one of the results may not even be possible to calculate.
::::::::::::::

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
