Computers are (not) precise {#precision_issues}
---------------------------

There are many differences between humans and computers, among those there is one that will keep haunting us in our journey: humans make calculations in "base 10" (decimal), computers make calculation in "base 2" (binary).

This requires computers to represent numbers differently, usually with the exponent+fraction representation (IEEE 754). Also computers have limited resources, thus have no concept of "infinity" (and conversely of "infinitesimal").

Let's assume a computer with a fixed (and reduced) precision and we execute the following C++ program (you can just copy it verbatim):

```{.cpp caption="A simple float precision test"}
#include <iostream>
#include <iomanip>

int main ()
{
    // This will reduce and fix the computer's precision for this execution
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

```{.bash}
g++ -Wall -Wextra -Werror -O0 precision_test.cpp -o precision_test.bin
```

This program will temporarily set a reduced precision in our number representation, and try to output the values of the numbers $1$, $0.1$ and $0.1^2=0.01$, let's see the results:

![Results of the simple float precision test](./images/computer_science/precision_1.png){width=50%}

:::: longdesc ::::
The image shows the results of the precision test as follows:

- This should be 1.0: 1
- This should be 0.1: 0.10000000149011611938
- This should be 0.01: 0.0099999997764825820923
- This should be true (1): 0
::::::::::::::::::

With the number $1$ it's all good, but... what is going on with $0.1$? What is all that garbage? The number $0.01$ is even worse! That's not even close! Why $0.1 + 0.1 + 0.1$ comes out as not $0.3$! **What is maths anymore?**

We have just met one of the (many) limitations of computers: computers cannot represent certain numbers without "approximating". Compilers and libraries exist to work around these issues, but we need to be ready to avoid surprises.

Just to reiterate: this is not a problem of the single programming language, we can see that C++ is affected, but also Python has the same issue:

![Python 2 has the same issues with precision as C++](./images/computer_science/precision_2.png){width=50%}

:::: longdesc ::::
Python 2.7.18's REPL shows that the expression `0.1+0.1+0.1==0.3` is False, due to how float numbers are represented in binary.
::::::::::::::::::

![Python 3 doesn't fare much better when it comes to precision](./images/computer_science/precision_3.png){width=50%}

:::: longdesc ::::
Python 3.9.5's REPL shows that the expression `0.1+0.1+0.1==0.3` is still False, due to how float numbers are represented in binary.
::::::::::::::::::

This is a computer issue in general: this may not be a huge problem for general use but, if we try to be too precise with our calculations, this may come back to bite us.


### Catastrophic cancellation

:::: wizardry ::::
Catastrophic cancellation is one of the many pitfalls that you may encounter when dealing with very small numbers. This doesn't happen really often in the world of game development, feel free to just skim through this mostly informative section.
::::::::::::::::::

With a name as dangerous-sounding as "catastrophic cancellation", this sure looks like a dangerous phenomenon, but it's only dangerous if we don't know what it is.

Catastrophic Cancellation (sometimes called "cancellation error") is an event that may happen when subtracting two (usually large) numbers that are close to each other in value.

**Warning:** from here on, in this section, there will be some technical language. I will try to make it as simple and understandable as possible.

Let's imagine a computer, such computer's memory can handle at most 8 decimals while its *[A.L.U.~\[g\]~](#gl_alu)* (the unit that takes case of "doing maths") can handle at most 16 decimal places.

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

We are off by 16\% of the total result, this is actually really bad.

What happened? If you look closely, the numbers are really close and even have 7 decimal digits in common, since our computer can memorize only 8 digits, the 9th to 13th decimal digits that looked so unimportant suddenly become a huge part of the result (due to the subtraction) but are already lost.

Random Numbers are not really random {#random}
------------------------------------

Computers are deterministic machines, given the same set of instructions and inputs, they will **always** return the same output. Someone may think about "random number generators" and sure, those programs look like they spit random numbers on your screen, but they actually don't.

The most important number when generating random numbers is called *seed* and it's the number used by the *generator* to produce random numbers.

Let's see an example of a random number generator in C++ (you can copy this program verbatim to try it):

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

```{.bash}
g++ -Wall -Wextra -Werror -O0 random_seed.cpp -o random_seed.bin
```

When we run the program, it will ask us to input a seed (which in our case is a number), after that it will just print 10 random numbers based on that seed. What would happen if we ran the program twice and use the same seed?

![Running a random number generator with the same seed will always output the same numbers](./images/computer_science/rng_seed.png){width=40%}

:::: longdesc ::::
A terminal showing the results of running the `./random_seed.bin` program twice. Both times the seed inserted is the number `14`, in both instances the program generates the same numbers.
::::::::::::::::::

Random numbers generated by computers are never truly random, that's why they are more properly called "pseudo-random numbers".

### How to seed a random number generator

From what we have seen earlier, the seed of our random number generator is something we need to be mindful about.

Choosing a static seed will make our game completely deterministic (if played in the same conditions), like we didn't use random numbers at all.

Some games use internal timers to see the random number generator, be it the time that the game has been running, the time that has passed from the beginning of the mission or something similar. This allows you to have some kind of "controlled RNG" that still has a bit of reproducibility.

Some choices expose the game to the possibility of RNG manipulation: where the player has partial or total control over the random number generator, by performing specific actions at specific times, for instance.

A very easy way to seed a generator is using the system time. Here's a more advanced random number generator that uses system time as its seed: if you run the program reasonably slow (not quicker than once a second) you will see the numbers changing.

```{.cpp caption="A random number generation program that uses system time as seed"}
#include<iostream>
#include<time.h>
int main(){
    // First of all we get the seed
    unsigned int seed = time(nullptr);
    // We print it for reference
    std::cout << "The current seed is:" << seed << std::endl;
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

We can save this program as `rand.cpp` compile this program with the following command:

```{.bash}
g++ -Wall -Wextra -Werror -O0 rand.cpp -o rand.bin
```

This is the result of the program being run twice, one second apart:

![Using the system time as RNG seed guarantees a degree of randomness](./images/computer_science/rng_time.png){width=40%}

:::: longdesc ::::
A terminal showing the results of running the `./rand.bin` program twice. The program is seeded automatically from the system time (first time the seed is `1667851777`, second time it is `1667851778`) and generates completely different series of numbers.
::::::::::::::::::
