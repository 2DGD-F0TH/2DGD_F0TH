Transformation Matrices
-----------------------

There will be a time, in our game development journey where we need to rotate an object, and that's bound to be pretty easy because rotation is something that practically all engines and tool kits do natively. But also there will be times where we need to do transformations by hand.

An instance where it may happen is rotating an item relative to a certain point or another item: imagine a squadron of war planes flying in formation, where all the planes will move (and thus rotate) relative to the "team leader".

In this chapter we'll talk about the 3 most used transformations:

- Stretching/Squeezing/Scaling;
- Rotation;
- Shearing.

And to do so, we will use the following reference image, complete with a quadrant of the Cartesian plane.

![Reference image for transformation matrices](./images/maths/transform_reference.svg){width=35%}

:::: longdesc ::::
Representation of the first quadrant of a Cartesian plane. There is a red square drawn on the plane with vertices at coordinates $(2, 2)$, $(5,2)$, $(5,5)$, $(2,5)$.
::::::::::::::::::

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

Stretching our reference image along the x and y axes respectively would look something like this:

![Stretching along the x and y axes](./images/maths/transform_stretch.svg){width=55%}

:::: longdesc ::::
The image is composed by two representation of the first quadrant of the Cartesian plane with a red rectangle drawn on. In the first, the rectangle has been stretched on the Y axis, moving its vertices to coordinates $(2,1)$, $(5,1)$, $(5,6)$ and $(2,6)$. In the second representation, the rectangle has been stretched along the X axis, moving its vertices to coordinates $(1,2)$, $(7,2)$, $(7,5)$ and $(1,5)$.
::::::::::::::::::

You can mix and match the factors and obtain different kinds of stretching, if the same factor $k$ is used both on the x and y-axis, we are performing a *scaling* operation, like follows:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = \begin{bmatrix}
    k & 0\\
    0 & k
\end{bmatrix} \begin{bmatrix}
    x\\
    y
\end{bmatrix}
$$

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

This could how our square could look, after a rotation:

![The result of applying a rotation matrix](./images/maths/transform_rotate.svg){width=35%}

:::: longdesc ::::
Representation of the first quadrant of a Cartesian plane. On the cartesian plane, a red square is drawn. The square has been rotated so that its vertices are located on coordinates $(3, 5.5)$, $(1.5, 3)$, $(4, 1.5)$ and $(5.5, 4)$.
::::::::::::::::::

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

The biggest problem in rotation is rotating an object relative to a certain point: you need to know the point of rotation $(x_p, y_p)$ in relation to the origin of the coordinate system you're using, and modify the matrices as follows:

$$
\begin{bmatrix}
    x'\\
    y'
\end{bmatrix} = T_R \begin{bmatrix}
    x - x_p\\
    y - y_p
\end{bmatrix} + \begin{bmatrix}
    x_p\\
    y_p
\end{bmatrix}
$$

In short, you need to rotate the item by first "bringing it centered to the origin", rotate it, and then bring it back into its original position.

### Shearing

During stretching, we used the elements that are in the "main diagonal" to stretch our objects. If we modify the elements in the "anti-diagonal", we will obtain shear mapping (or shearing).

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

Here is how shearing would look, as an example:

![Shearing along the x and y axes](./images/maths/transform_shear.svg){width=55%}

:::: longdesc ::::
The image is composed by two representation of the first quadrant of the Cartesian plane with a red rectangle drawn on. In the first, the rectangle has been sheared on the X axis, moving its vertices to coordinates $(1.5,2)$, $(4.5, 2)$, $(5.5,5)$ and $(2.5,5)$. In the second representation, the rectangle has been sheared along the Y axis, moving its vertices to coordinates $(2,2.5)$, $(5,1.5)$, $(5,4.5)$ and $(2,5.5)$.
::::::::::::::::::
