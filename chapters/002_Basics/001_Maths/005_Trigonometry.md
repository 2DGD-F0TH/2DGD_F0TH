Trigonometry
------------

When you want to develop a game, you will probably find yourself needing to rotate items relative to a certain point or relative to each other. To do so, you need to know a bit of trigonometry, so here we go!

### Radians vs Degrees

In everyday life, angles are measured in degrees, from 0 to 360 degrees. In some situations in maths, it is more comfortable to measure angles using radians, from 0 to $2 \pi$.

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

![Unit Circle definition of sine and cosine](./images/maths/sincos.svg){width=40%}

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

As with practically all maths formulas, there are inverse formulas for sine and cosine, called $arcsin$ and $arccos$, which allow to find an angle, given its sine and cosine.

In this book we won't specify more, besides what could be the most useful: the 2-argument arctangent.

This formula allows you to find the angle of a vector, relative to the coordinate system, given the `x` and `y` coordinates of its "tip", such angle $\theta$ is defined as:

$$ \theta = arctan(\frac{y}{x}) $$

![Graphical plotting of the angle of a vector](./images/maths/arctan2.svg){width=40%}

Numerical Analysis
------------------

Here we will give some pointers over some algorithms and methods that may be useful to better explain some topics treated in this book. Feel free to skip or quickly read this section if you don't want to dive into too much detail over this kind of maths.

### Newton-Raphson method {#newtonmethod}

:::: wizardry ::::
This section treats of how to approximate a function value in an iterative way. This will be useful to know what the "Fast Inverse Square Root" algorithm uses. Feel free to skim through this section.
::::::::::::::::::

Also known as Newton's method, this is an iterative algorithm that is used to get progressively better approximations to the roots of a function.

The algorithm starts with a "guess", called $x_0$, and produces the first approximation using the formula:

$$ x_1 = x_0 - \frac{f(x_0)}{f'(x_0)}$$

Each subsequent guess (and thus iteration) can be obtained similarly by using the formula:

$$ x_{n+1} = x_n - \frac{f(n)}{f'(n)}$$

And such guess will be more precise than the previous one (if we don't consider some situations where approaching the root can be problematic or not possible). The algorithm will stop when you reach an approximation that is "good enough".

Obviously all limitations of standard functions apply, such as domain and trouble with divisions by zero.

Coordinate Systems on computers
---------------------------------

When it comes to 2D graphics on computers, our world gets quite literally turned upside down.

In our maths courses we learned about the Coordinate Plane, with an origin and an `x` axis going from left to right and a `y` axis going from bottom to top, where said axis cross it's called the "Origin".

![Image of a coordinate plane](images/maths/coord.svg){width=30%}

When it comes to 2D graphics on computers and game development, the coordinate plane looks like this:

![Image of a screen coordinate plane](images/maths/screen_coord.svg){width=30%}

The origin is placed on the top left of the screen (at coordinates `(0,0)`) and the `y` axis is going from top to bottom. It's a little weird at the beginning, but it's not hard to get used to it.
