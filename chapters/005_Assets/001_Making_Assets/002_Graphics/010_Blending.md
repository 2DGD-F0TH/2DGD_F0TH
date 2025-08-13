### Blending surfaces

Sometimes we need sprites (or textures) to influence each other in a different way than simple "draw `x` instead of `y`". This may be useful for applying ambient lighting and shadow effects to sprites.

Doing so on a pixel-by-pixel basis is inefficient (in a similar way to what [ray casting](#raycasting) does with collision detection), thus it's better to leave such task to the hardware, usually the GPU.

Thus the concept of "surface blending" was born: a way to take two surfaces and have a result that is essentially a mathematical operation of the single pixel colors.

This is usually supported by the engine or library that you use, and doesn't need to be manually programmed.

#### Addition

Surface addition is very simple: each pixel color (usually as an RGB 3-tuple) from the first surface is taken and each channel value is summed to the values of the second surface (with a maximum value of $255$ for each channel).

So if we were to take two overlapping rectangles, one red (with RGB tuple $(255, 0, 0)$) and one green (with RGB tuple $(0, 255, 0)$):

![Surface blending - addition (1/2)](./images/resources/blending/addition_1.svg){width=30%}

We can take the tuples and calculate the resulting color:

$$
\begin{array}{r r r r}
  & 255 & 0   & 0 \\
+ & 0   & 255 & 0\\ \hline
  & 255 & 255 & 0
\end{array}
$$

The resulting color is yellow.

![Surface blending - addition (2/2)](./images/resources/blending/addition_2.svg){width=30%}

So if you were to use a gray sprite as your second surface (which usually has all 3 channels set at the same value), the resulting surface will have lighter pixels where the two sprites overlap, creating a "lighting effect".

![Simple lighting effect using 2 surfaces and blending](./images/resources/blending/addition_light.svg){width=50%}

This approach is a little bit limited: you need to align the surfaces correctly, or artifacts may be visible, but it's not that hard.

#### Subtraction

{{placeholder}}

<!-- TODO: Speak about blending surfaces via subtractive blending -->

#### Alpha Blending

{{placeholder}}

<!-- TODO: Speak about blending surfaces via alpha blending -->
