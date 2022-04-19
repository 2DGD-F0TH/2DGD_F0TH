Simplifying your conditionals with Karnaugh Maps
------------------------------------------------

Karnaugh maps are a useful tool to simplify boolean algebra expressions, as well as identifying and potentially solving race conditions.

The output of a Karnaugh Map will always be an "OR of ANDs".

The best way to explain them is to give an example.

Let's take the following truth table:

| A | B |  $f$  |
|:-:|:-:|:-----:|
| 0 | 0 | $0$   |
| 0 | 1 | $1$   |
| 1 | 0 | $1$   |
| 1 | 1 | $0$   |

: The first truth table we'll simplify with Karnaugh Maps

Said table can contain any number of variables (we'll see how to implement those). To be precise, this table represents the formula `A XOR B` (XOR means 'exclusive or').

Let's arrange it into a double-entry table, like this (Values of A are on top, values of B are on the left):

![Karnaugh Map for A XOR B](./images/computer_science/karnaugh/karnaugh_1.svg){width=20%}

Now we have to identify the biggest squares or rectangles that contain 2^n^ elements equal to 1 so that we can cover all the "1" values we have (they can overlap). In this case we're unlucky as we have only two small rectangles that contain one element each:

![Karnaugh Map where the elements of the two "rectangles" have been marked green and red](./images/computer_science/karnaugh/karnaugh_2.svg){width=20%}

In this case, we have the result we want with the following formula: $f = (A \land \bar{B}) \lor (\bar{A} \land B)$

Not an improvement at all, but that's because the example is a really simple one.

### "Don't care"s

Karnaugh Maps show more usefulness when we have the so-called "don't care"s, situations where we don't care (wow!) about the result. Here's an example.

| A | B |  $f$  |
|:-:|:-:|:-----:|
| 0 | 0 | $0$   |
| 0 | 1 | $1$   |
| 1 | 0 | $1$   |
| 1 | 1 | $x$   |

: Truth table with a "don't care" value

Putting this truth table into a Karnaugh map we get something a bit more interesting:

![Karnaugh Map with a "don't care" value](./images/computer_science/karnaugh/karnaugh_3.svg){width=20%}

Now we have a value that behaves a bit like a "wild card", that means we can pretend it's either a 0 or 1, depending on the situation. In this example we'll pretend it's a 1, because it's the value that will give us the biggest "rectangles".

![Karnaugh Map where we pretend the "don't care" value is equal to 1](./images/computer_science/karnaugh/karnaugh_4.svg){width=20%}

Now we can find two two-elements rectangles in this map.

The first is the following one:

![First Rectangle in the Karnaugh map](./images/computer_science/karnaugh/karnaugh_5.svg){width=20%}

In this case, we can see that the result is 1 when $B=1$, no matter the value of A. We'll keep this in mind.

The second rectangle is:

![Second Rectangle in the Karnaugh map](./images/computer_science/karnaugh/karnaugh_6.svg){width=20%}

In this case, we can see that the result is 1 when $A=1$, no matter the value of B.

This translates into a formula of: $f = (A) \lor (B)$, considering that we don't care about the result that comes out when $A=1$ and $B=1$.

:::: note ::::
If instead of 1, we ended up choosing 0 for our "don't care", we would have obtained $f = (A) \land (B)$. For our necessities, this would have been a good solution too.
::::::::::::::

### A more complex map

When we have more variables, like the following truth table:

| A | B | C | D |  $f$  |
|:-:|:-:|:-:|:-:|:-----:|
| 0 | 0 | 0 | 0 |  $0$  |
| 0 | 0 | 0 | 1 |  $0$  |
| 0 | 0 | 1 | 0 |  $0$  |
| 0 | 0 | 1 | 1 |  $0$  |
| 0 | 1 | 0 | 0 |  $0$  |
| 0 | 1 | 0 | 1 |  $0$  |
| 0 | 1 | 1 | 0 |  $1$  |
| 0 | 1 | 1 | 1 |  $0$  |
| 1 | 0 | 0 | 0 |  $1$  |
| 1 | 0 | 0 | 1 |  $1$  |
| 1 | 0 | 1 | 0 |  $1$  |
| 1 | 0 | 1 | 1 |  $1$  |
| 1 | 1 | 0 | 0 |  $1$  |
| 1 | 1 | 0 | 1 |  $1$  |
| 1 | 1 | 1 | 0 |  $1$  |
| 1 | 1 | 1 | 1 |  $x$  |

Now we'll have to group up our variables and put them in a Karnaugh Map using Gray Code, practically each row or column differs from the adjacent ones by only one bit.

The resulting Karnaugh map is the following (AB on columns, CD on rows):

![A more complex Karnaugh map](./images/computer_science/karnaugh/karnaugh_7.svg){width=30%}

We can see two rectangles that contain 2^n^ items, one with 2 items, the other with 8, considering the only "don't care" value as 1.

![First rectangle of the more complex Karnaugh map](./images/computer_science/karnaugh/karnaugh_8.svg){width=30%}

In this first rectangle, we can see that the values of C and D don't matter towards the result, as well as the value of B. The only variable that gives the result on this rectangle is $A=1$. We'll keep that in mind

Let's see the second rectangle:

![Second rectangle of the more complex Karnaugh map](./images/computer_science/karnaugh/karnaugh_9.svg){width=30%}

In this case A doesn't give any contribution to the result, but at the same time we need $B=1$, $C=1$ and $D=0$ to get the wanted result.

$D=0$ translates into $\bar{D}=1$, which brings the formula to: $f = A \lor (B \land C \land \bar{D})$.

If we didn't have that "don't care" value, everything would have been more complex.

### Guided Exercise

Let's remove the "don't care" value and have the following truth table:

| A | B | C | D |  $f$  |
|:-:|:-:|:-:|:-:|:-----:|
| 0 | 0 | 0 | 0 |  $0$  |
| 0 | 0 | 0 | 1 |  $0$  |
| 0 | 0 | 1 | 0 |  $0$  |
| 0 | 0 | 1 | 1 |  $0$  |
| 0 | 1 | 0 | 0 |  $0$  |
| 0 | 1 | 0 | 1 |  $0$  |
| 0 | 1 | 1 | 0 |  $1$  |
| 0 | 1 | 1 | 1 |  $0$  |
| 1 | 0 | 0 | 0 |  $1$  |
| 1 | 0 | 0 | 1 |  $1$  |
| 1 | 0 | 1 | 0 |  $1$  |
| 1 | 0 | 1 | 1 |  $1$  |
| 1 | 1 | 0 | 0 |  $1$  |
| 1 | 1 | 0 | 1 |  $1$  |
| 1 | 1 | 1 | 0 |  $1$  |
| 1 | 1 | 1 | 1 |  $0$  |

Let's put it into a Karnaugh Map:

![Guided Exercise: Karnaugh Map (1/4)](./images/computer_science/karnaugh/karnaugh_10.svg){width=30%}

Find the biggest rectangles:

![Guided Exercise: Karnaugh Map (2/4)](./images/computer_science/karnaugh/karnaugh_11.svg){width=30%}

![Guided Exercise: Karnaugh Map (3/4)](./images/computer_science/karnaugh/karnaugh_12.svg){width=30%}

![Guided Exercise: Karnaugh Map (4/4)](./images/computer_science/karnaugh/karnaugh_13.svg){width=30%}

Extract the result: $f = (A \land \bar{C}) \lor (A \land \bar{B}) \lor (B \land C \land \bar{D})$
