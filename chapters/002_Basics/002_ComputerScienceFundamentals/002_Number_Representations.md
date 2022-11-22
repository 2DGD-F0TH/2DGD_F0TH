Number representations
----------------------

When you work with computers, it's impossible to avoid learning a bit of number representations. Computers work with a different logic than humans do: humans have complex minds and thoughts, while most of the time computers work in ones and zeroes. Most of what we see on a screen can be reduced to electrons going through a semiconductor in kind of an orderly fashion: changing from 0 volts (ground) to 5 volts.

### The most used representations

Here we will take a quick look at the most used representations. Some are more fundamental than others, but they are all useful in their own way.

Each representation will use a subscript to represent its representation. If no subscript is present it means the standard decimal representation is used.

#### Decimal

This is the standard decimal notation everyone is used to, we have 10 digits at our disposal:

$$
0\ 1\ 2\ 3\ 4\ 5\ 6\ 7\ 8\ 9
$$

And we place them in certain positions (units, tens, hundreds, thousands, etc...) to represent a certain quantity. We will use this as a basis for all other representations.

So if you want to represent $9 + 1$ you will use the $1$ digit, followed by the $0$ digit to make $10$.

#### Binary

This is the most used representation in computer science, we have only two digits at our disposal: $0\ 1$.

Thus if you want to make $1_{bin}+1_{bin}$, you will have to use the $1$ digit, followed by the $0$ digit, thus making $10_{bin}$, which is the binary representation of 2.

Here are the first 10 numbers for comparison purposes:

| Decimal | Binary |
| :-----: | :----: |
| 0       | 0      |
| 1       | 1      |
| 2       | 10     |
| 3       | 11     |
| 4       | 100    |
| 5       | 101    |
| 6       | 110    |
| 7       | 111    |
| 8       | 1000   |
| 9       | 1001   |
| 10      | 1010   |

Table: Comparison between decimal and binary representations

Binary numbers can be used at low level to represent any kind of "binary condition" too: yes/no, true/false are usually mapped to 1 and 0 respectively. This will probe useful in some cases where we will use "binary numbers" to represent groups of "binary conditions" in a compact way, but that's an advanced thing we'll see later.

#### Octal

In the octal representation we have 8 digits at our disposal:

$$
0\ 1\ 2\ 3\ 4\ 5\ 6\ 7
$$

Thus the representation of the decimal number $8$ in the octal system is $10_{oct}$.

The octal number system doesn't find much use in computer science besides being a quicker way to represent binary numbers. The conversion is quite easy and will be explained in a bit.

Here's a quick comparison between decimal and octal representations:

| Decimal | Octal  |
| :-----: | :----: |
| 0       | 0      |
| 1       | 1      |
| 2       | 2      |
| 3       | 3      |
| 4       | 4      |
| 5       | 5      |
| 6       | 6      |
| 7       | 7      |
| 8       | 10     |
| 9       | 11     |
| 10      | 12     |

Table: Comparison between decimal and octal representations

#### Hexadecimal

Hexadecimal is definitely the second most used representation in computer science, due to how easy it is to represent 4 bytes in a very compact notation.

In the hexadecimal notation, we have sixteen digits at our disposal:

$$
0\ 1\ 2\ 3\ 4\ 5\ 6\ 7\ 8\ 9\ A\ B\ C\ D\ E\ F
$$

Here's a table of the first 20 numbers to clarify a bit how things work:

| Decimal | Hex    |
| :-----: | :----: |
| 0       | 0      |
| 1       | 1      |
| 2       | 2      |
| 3       | 3      |
| 4       | 4      |
| 5       | 5      |
| 6       | 6      |
| 7       | 7      |
| 8       | 8      |
| 9       | 9      |
| 10      | A      |
| 11      | B      |
| 12      | C      |
| 13      | D      |
| 14      | E      |
| 15      | F      |
| 16      | 10     |
| 17      | 11     |
| 18      | 12     |
| 19      | 13     |
| 20      | 14     |

Table: Comparison between decimal and hexadecimal representations

### Converting between decimal and binary

The algorithm to convert between decimal and binary is quite simple. It is an iterative algorithm that consists in integer dividing the number by 2, until the result of the division is 1. The modulo of such divisions will make up our binary number.

An example is worth a thousand words: let's convert the number 38 to binary.

First of all, we integer divide 38 by two: the result is 19, there is no remainder, so we'll use zero.

| Dividend | Remainder |
| -------: | :-------- |
| 38       | 0         |
| 19       |           |

Let's continue: we integer divide 19 by two: the result is 9, with 1 as remainder.

| Dividend | Remainder |
| -------: | :-------- |
| 38       | 0         |
| 19       | 1         |
| 9        |           |

We iterate some more, by integer dividing until we get 1 as a dividend, at that point we make the last division, which will have remainder 1:

| Dividend | Remainder |
| -------: | :-------- |
| 38       | 0         |
| 19       | 1         |
| 9        | 1         |
| 4        | 0         |
| 2        | 0         |
| 1        | 1         |

Now we just need to read our remainders from bottom to top. So the binary representation of $38_{dec}$ is $100110_{bin}$.

:::: note ::::
This is actually a much more generic algorithm: you can convert from decimal to octal and hexadecimal for instance, just by dividing by 8 and 16 respectively. You can convert 38 to octal and hexadecimal as an exercise: the results are $46_{oct}$ and $26_{hex}$.
::::::::::::::

#### Two's complement

So far we've seen how to convert positive integers from decimal to binary, but how do we represent negative integers?

That's where "two's complement" representation comes into play: there is a bunch of theory behind why it's called this way, and how it works, but what we need to know will be how to represent a negative number.

> To represent a negative binary number in two's complement you invert all the bits of such number, then add 1.

As usual, an example is worth a thousand words. We want to convert the number $-38$ into binary.

First of all we need to define what our range of numbers will be, so that we know how many bits we will use. This is done because this range will be equally split between positive and negative numbers. In this example I will choose a normal 8-bit representation, which can represent numbers spacing from -128 to 127.

The first step is to convert $38$ to binary, which as we saw is $100110_{bin}$. We will pad this binary number to 8 bits, obtaining $00100110_{bin}$ as a result.

Now we just need to invert the all the bits in the number, obtaining $11011001_{bin}$ as a result.

Last step is adding 1 to what we got in the previous step, thus the final result is $11011010_{bin}$.

:::: note ::::
The more perceptive of you may have noticed a problem: what if we tried to represent the number 128 with 8 bits?

We would obtain $1000000_{bin}$ which is actually the representation of -128 in two's complement. This is called an "integer overflow", so be careful when mixing unsigned and signed integers.
::::::::::::::


#### Floating point

{{placeholder}}

<!-- TODO: Talk briefly about exponent+fraction representation and the lack of precision it may bring -->

### Converting between binary and octal

As mentioned before, octal can be used as a "shorthand way" to represent binary. The conversion is pretty simple.

> To convert from binary to octal, take the binary digits in groups of 3 (with the necessary padding) and convert them in octal. Then just "stick them together".

Let's take our number 38, it has the following representation:

$$
100\ 110_{bin}
$$

$100_{bin}$ converts to $4_{oct}$, while $110_{bin}$ converts to $6_{oct}$. If we stick them together we obtain the final result: $46_{oct}$.


### Gray Code

:::: wizardry ::::
Gray code isn't really used in game development, but it will be briefly explained here since it will be used in [Karnaugh Maps](#karnaugh)
::::::::::::::::::

Gray code (sometimes known as "reflected binary code") is a particular ordering of the binary system where two successive values differ by only one bit.

Gray code is used in many fields, from Digital (and cable) TV (for error-correction) to analog to digital conversion. In this book we will use it as a representation inside [Karnaugh Maps](#karnaugh).

Here is a simple representation of the first 10 numbers in decimal, binary and gray code:

| Decimal | Binary | Gray Code |
| :-----: | :----: | :-------: |
| 0       | 0000   | 0000      |
| 1       | 0001   | 0001      |
| 2       | 0010   | 0011      |
| 3       | 0011   | 0010      |
| 4       | 0100   | 0110      |
| 5       | 0101   | 0111      |
| 6       | 0110   | 0101      |
| 7       | 0111   | 0100      |
| 8       | 1000   | 1100      |
| 9       | 1001   | 1101      |
| 10      | 1010   | 1111      |
