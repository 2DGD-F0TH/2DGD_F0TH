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

The sum vector $s$ will then be:

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

### "Clamping" a Vector

This is not an operation "per se", but there are occasions where we need to limit the length of a vector: this usually happens when we are working with velocity, as not limiting it would allow an object to change position faster and faster, making the game less playable and even [breaking time-stepping collision detection algorithms](#bulletthroughpaper).

To clamp a vector, we need to find its magnitude and direction first, which is the "normalized vector". Let's think about the vector $v$, its magnitude and direction are:

$$ ||v|| = \sqrt{v \cdot v} $$
$$ \hat{v} = \frac{v}{||v||} $$

After that, we can build a new vector using the "clamped magnitude" (which we'll call $||v||_{clamp}$), calculated as such:

$$
||v||_{clamp}=\begin{cases}
||v||\ when\ ||v|| < ||v||_{max}\\
||v||_{max}\ otherwise
\end{cases}
$$

To build the new vector, we just need to multiply $||v||_{clamp}$ by $\hat{v}$:

$$ v = ||v||_{clamp} \cdot \hat{v} $$

The new vector will have the same direction as the old one, but its magnitude will be clamped, just like we wanted.
