{{pagebreak}}

Some Computer Science Fundamentals
===================================

:::::: {.epigraph author="Edsger W. Dijkstra"}
The computing scientist's main challenge is not to get confused by the complexities of his own making.
::::::

In order to understand some of the language that is coming up, it is necessary to learn a bit of the computer science language and fundamentals.

This chapter will briefly explain some of the language and terms used, their meaning and how they contribute to your activity of developing games.

In this chapter we'll assume you already know what the following terms mean:

- Truth Table
- Algorithm

Recursion
---------

Starting from the (arguably hard) theme of recursion may seem weird, but it is important to understand recursion as soon as possible so we can make the best use of it.

There is a joke I like telling around about recursion:

> To understand recursion, you must first understand recursion.

What is recursion? Recursion is the usage of a function that calls itself.

Your first question will probably be: wouldn't that make the program lock up forever in some kind of loop? It may. But if you're careful, recursion is an amazing tool that allows you to earn a lot of clarity and brevity.

Let's imagine a simple algorithm: we want to make our program count backwards from a number `n` to 0. In a simple "loop" fashion, we may write the following:

```{src='computer_science/recursion_loop' caption='Counting from n to 0 using a loop'}
```

Pretty simple, right? A real-world example would be counting back from 10 to 0: we print 10, we subtract 1 to get 9, we print 9, subtract 1 to get 8, ...

Let's turn our thinking around for a second. We can see counting back from to 10 to 0 like this: we print 10 and then we count backwards from 9. Counting backwards from 9 would just mean printing 9 and then counting back from 8, etc...

We just turned our simple loop into a recursive function:

```{src='computer_science/recursion' caption='Counting from n to 0 using recursion'}
```

Recursive functions have three main components:

- A **base case** (sometimes called a "stop condition"): this allows the function to stop calling itself when a certain condition is reached;
- A **procedure** that elaborates on data or simply does something (in our example, it just prints the number);
- A **recursive call** to the same function we are writing, the call is done in a way that every call gets closer to the "stop condition". It can be done by calling the function on a subset of its argument (if it is a list), until the list has only 1 item or on a smaller number (if the function argument is a number instead).

Recursion can be classified in many ways:

- **By the number of recursive calls:** single vs multiple recursion;
- **By how the recursive call is made:** direct (a function calls itself directly) vs indirect (a function A is called by another function B, which in turn is called by function A)
- **By the position of the recursive call:** head vs tail recursion.

I want to underline the last distinction: what we've seen in the previous listing is called "tail recursion": the recursive call is done **after** everything else (the procedure).

Head recursion is instead done when the recursive call is done **before** the procedure starts, so we can transform our "count down" function to a "count up" just by switching from "tail" to "head" recursion and adding a print statement.

```{src='computer_science/head_recursion' caption='Counting from 0 to n using head recursion'}
```

Computers are (not) precise {#precision_issues}
---------------------------

There are many differences between humans and computers, among those there is one that will keep haunting us in our journey: humans make calculations in "base 10" (decimal), computers make calculation in "base 2" (binary).

This requires computers to represent numbers differently, usually with the exponent+fraction representation (IEEE 754). Also computers have limited resources, thus have no concept of "infinity" (and conversely of "infinitesimal").

Let's assume a computer with a reduced precision and we execute the following C++ program:

```{.cpp caption="A simple float precision test"}
#include <iostream>
#include <iomanip>

int main ()
{
    // This will reduce the computer's precision for this execution
    std::cout << std::setprecision(20);

    float d1(1.0);
    std::cout << "This should be 1.0: " << d1 << std::endl;

    float d2(0.1);
    std::cout << "This should be 0.1: " << d2 << std::endl;

    float d3(0.1*0.1);
    std::cout << "This should be 0.01:" << d3 << std::endl;

    bool x (0.1 + 0.1 + 0.1 == 0.3);
    std::cout << "This should be true (1): " << x << std::endl;

    return 0;
  }
```

We save it as "precision_test.cpp" and compile it with the following command line (on Linux):

```{.sh}
g++ -Wall -Wextra -Werror -O0 precision_test.cpp -o precision_test.bin
```

This program will temporarily set a reduced precision in our number representation, and try to output the values of the numbers $1$, $0.1$ and $0.1^2=0.01$, let's see the results:

![Results of the simple float precision test](./images/computer_science/precision_1.png){width=50%}

With the number $1$ it's all good, but... what is going on with $0.1$? What is all that garbage? The number $0.01$ is even worse! That's not even close! Why $0.1 + 0.1 + 0.1$ comes out as not $0.3$! **What is maths anymore?**

We have just met one of the (many) limitations of computers: computers cannot represent certain numbers without "approximating". Compilers and libraries exist to work around these issues, but we need to be ready to avoid surprises.

Just to reiterate: this is not a problem of the single programming language, we can see that C++ is affected, but also Python has the same issue:

![Python 2 has the same issues with precision as C++](./images/computer_science/precision_2.png){width=50%}

![Python 3 doesn't fare much better when it comes to precision](./images/computer_science/precision_3.png){width=50%}

This is a computer issue in general: this may not be a huge problem for general use but, if we try to be too precise with our calculations, this may come back to bite us.


### Catastrophic cancellation

With a name as dangerous-sounding as "catastrophic cancellation", this sure looks like a dangerous phenomenon, but it's only dangerous if we don't know what it is.

Catastrophic Cancellation (sometimes called "cancellation error") is an event that may happen when subtracting two (usually large) numbers that are close to each other in value.

**Warning:** from here on, in this section, there will be some technical language. I will try to make it as simple and understandable as possible.

Let's imagine a computer, such computer's memory can handle at most 8 decimals while its A.L.U. (the unit that takes case of "doing maths") can handle at most 16 decimal places.

Now let's take two numbers:

$$x=0.5654328749846\ y=0.5654328510104$$

When we transfer such numbers in our memory, the computer will approximate such numbers to fit in its memory constraints. We'll represent that by applying to each number a function $fl()$ that we can read as "float representation of this number". So we'll end up having:

$$fl(x)=0.56543287\ fl(y)=0.56543285$$

This is generally called an "assignment error", where during the assignment to a variable, a number loses part of its information.

Let's try an calculate how off those approximations are (by calculating the percent "relative error"), just to get an idea of what we lost by just loading the numbers on our "fake computer":

$$\delta_x = \frac{| x - fl(x) |}{x} = ~ 0.00000088\%$$
$$\delta_y = \frac{| y - fl(y) |}{y} = ~ 0.00000017\%$$

We can see that our approximations are **very close** to the numbers we want to calculate, now let's calculate $x-y$. Making things by hand we would have:

$$x - y = 0.239772 \times 10^{-7}$$

That's a tiny number right there. Now let's calculate $fl(x) - fl(y)$, remembering that the A.L.U. will fill up to 16 decimals:

$$fl(x) - fl(y) = 0.5654328700000000 - 0.5654328500000000 = 0.0000000200000000 = 0.2 \times 10^{-7}$$

That doesn't look so bad, unless we look at the "relative error":

$$\delta = \frac{| 0.239772 \times 10^{-7} - 0.2 \times 10^{-7} |}{0.239772 \times 10^{-7}} = 16.6\%$$

Oh no... We're off by 16\% of the total result! That's a huge loss! A real catastrophe.

What happened? If you look closely, the numbers are really close and even have 7 decimal digits in common, since our computer can memorize only 8 digits, the 9th to 13th decimal digits that looked so unimportant suddenly become a huge part of the result (due to the subtraction) but are already lost.

Random Numbers are not really random {#random}
------------------------------------

Computers are deterministic machines, given the same set of instructions and inputs, they will **always** return the same output. Someone may think about "random number generators" and sure, those programs look like they spit random numbers on your screen, but they actually don't.

The most important number when generating random numbers is called *seed* and it's the number used by the *generator* to produce random numbers.

Let's see an example of a random number generator in C++:

```{.cpp caption="A simple random number generation program"}
#include<iostream>
int main(){
    // First of all we get the seed
    unsigned int seed;
    std::cout << "Type the seed: " ;
    std::cin >> seed;
    // Now we seed the randomizer
    srand(seed);
    // Small presentation
    std::cout << "This generator will now generate 10 random numbers" << std::endl;
    // Output 10 random numbers
    for (int i = 0; i < 10; ++i) {
        std::cout << rand() << std::endl;
    }
    // Finish the program
    return 0;
}
```

We can save this program as `random_seed.cpp` compile this program with the following command:

```{.sh}
g++ -Wall -Wextra -Werror -O0 random_seed.cpp -o random_seed.bin
```

When we run the program, it will ask us to input a seed (which in our case is a number), after that it will just print 10 random numbers based on that seed. What would happen if we ran the program twice and use the same seed?

![Running a random number generator with the same seed will always output the same numbers](./images/computer_science/rng_seed.png){width=40%}

Random numbers generated by computers are never truly random, that's why they are more properly called "pseudo-random numbers".

De Morgan's Laws and Conditional Expressions
--------------------------------------------

De Morgan's laws are fundamental in computer science as well as in any subject that involves propositional logic. We will take a quick look at the strictly coding-related meaning.

De Morgan's laws can be written as:

> not (A and B) = not A or not B
>
> not (A or B) = not A and not B

In symbols:

$$ \overline{(A \land B)} = \bar{A} \lor \bar{B} $$
$$ \overline{(A \lor B)} = \bar{A} \land \bar{B} $$

These laws allow us to express our own conditionals in different ways, allowing for more readability and maybe avoid some boolean manipulation that can hinder the performance of our game.
