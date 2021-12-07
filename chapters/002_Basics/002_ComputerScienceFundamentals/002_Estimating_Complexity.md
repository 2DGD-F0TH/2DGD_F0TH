Estimating the complexity of algorithms
---------------------------------------

Now more than ever, you need to be able to be efficient. How do you know how "efficient" some piece of algorithm is?

Seeing how much time it takes is not an option, computer specifications change from system to system, so we need something that could be considered "cross-platform".

This is where notations come into play.

There are 3 types of Asymptotic notation you should know: $\Omega$, $\Theta$ and O.

**$\Omega$()** represents **a lower bound**: this means that the algorithm will take **at least** as many cycles as specified.

**O()** represents **an upper bound**: it's the most used notation and means that the algorithm will take **at most** as many cycles as specified.

**$\Theta$()** is a **tight bound**, used when the big-O notation and the big-$\Omega$ notation have the same value, which can help define the behavior of the algorithm better.

We will now talk about the most common Big-O notations, from "most efficient" to "least efficient".

:::: pitfall ::::
Be mindful of one specific thing: these notations simply tie how the algorithm performs in relation to how a certain variable grows (usually a dataset). If you know for certain that a dataset stays relatively small, a less efficient algorithm may not make a huge difference.
::::

### O(1)

An algorithm that executes in **O(1)** is said to execute "in constant time", which means that no matter how much data is input in the algorithm, said algorithm will execute in the same time.

An example of a simple O(1) algorithm is an algorithm that, given a list of elements (with at least one element), returns `True` if the first element is `null`.

```{src='computer_science/o1' caption='Example of an O(1) algorithm'}
```

To be precise, this algorithm will perform both in O(1) and $\Omega(1)$, so it will perform in $\Theta(1)$.

### O(log(n))

An algorithm that executes in O(log(n)) is said to execute in "logarithmic time", which means that given an input of **n** items, the algorithm will execute **log(n)** cycles at most.

An example of a O(log(n)) algorithm is the so-called "binary search" on a ordered list of items.

```{src='computer_science/binary_search' caption='Example of an O(log(n)) algorithm (Binary Search)'}
```

The best case is the time when you get the element to find to be the "middle element" of the list, in that case the algorithm will execute in linear time: $\Theta(1)$ - You need **at least one lookup** ($\Omega(1)$) and **at most one lookup** ($O(1)$).

In the worst case, the element is not present in the list, so you have to split the list and find the middle element until you realize that you don't have any more elements to iterate - this translates into a **tight bound** of $\Theta(log_{2}n)$

### O(n)

An algorithm that executes in O(n) is said to execute in "linear time", which means that given an input of **n** items, the algorithm will execute at most **n** cycles.

An example of a simple O(n) algorithm is the one that prints a list, element by element.

```{src='computer_science/printlist' caption='Example of an O(n) algorithm (printing of a list)'}
```

It's evident that this algorithm will call the `print` function `n` times, where `n` is the size of the list. This translates in a $\Theta(n)$ complexity, which is both $O(n)$ and $\Omega(n)$.

There is no "best" or "worst" case here, the algorithm prints `n` elements, no matter their order, the alignment of planets and stars or the permission of its parents.

### O(n·log(n))

An algorithm that executes in O(n·log(n)) executes in a time slightly longer than a linear algorithm, but it's still considered "ideal". These algorithms are said to execute in "quasi-linear", "log-linear", "super-linear" or "linearithmic" time.

Given an input of **n** elements, these algorithms execute **n·log(n)** steps, or cycles.

Some algorithms that run in O(n·log(n)) are:

- Quick Sort
- Heap Sort
- Fast Fourier Transforms (F.F.T.)

These algorithms are more complex than a simple example and would require a chapter on their own, so we'll leave examples aside for now.

### O(n^2^)

Quadratic algorithms, as the algorithms that execute in O(n^2^) are called, are the door to the "danger zone".

These algorithms can eat your CPU time quite quickly, although they can still be used for small computations somewhat efficiently.

Given an input of **n** elements, these algorithms execute **n^2^** cycles, which means that given an input of **20** elements, we'd find ourselves executing **400** cycles.

A simple example of a quadratic algorithm is "bubble sort". A pseudo-code implementation is written here.

```{src='computer_science/bubblesort' caption='Example of an O(n²) algorithm (bubble sort)'}
```

Anything with complexity higher than O(n^2^) is usually considered unusable.

### O(2^n^)

Algorithms that execute in exponential time are considered a major code red, an will usually be replaced with heuristic algorithms (which trade some precision for a lower complexity).

Given an input of `20` elements, an algorithm that executes in O(2^n^) will execute 2^20^ = 1 048 576 cycles!


A primer on calculating the order of your algorithms
-----------------------------------------------------

### Some basics

When you estimate an algorithm, you usually want to calculate how it functions "in the worst case", which usually means that all loops get to their end (of the list or the counter) and everything takes the longest time possible.

Let's start with an example:

```{src='computer_science/bigo/bigo1' caption='A simple O(1) algorithm'}
```

This is a simple assignment operation, we are considering this instantaneous. So its complexity is $O(1)$.

Now let's see another algorithm:

```{src='computer_science/bigo/bigo2' caption='A simple o(n) algorithm'}
```

In this case we are iterating through a list, we can see that as the list grows, the number of times we print an element on our screen grows too. So if the list is $n$ items long, we will have $n$ calls to the output statement. This is an $O(n)$ complexity algorithm.

Now let's take something we already saw and analyze it: the bubble sort algorithm:

```{src='computer_science/bubblesort' caption='The bubble sort algorithm, an O(n²) algorithm'}
```

This will require a small effort on our part: we can see that there are 2 nested loops in this code. What's our worst case? The answer is "The items are in the reverse order".

When the items are in the reverse order, we will need to loop through the whole list to get the biggest item at the end of the list, then another time to get the second-biggest item on the second-to-last place on the list... and so on.

So every time we bring an item to its place, we iterate through all the list once. This happens for each item.

So, in a list of length "n", we bring the biggest item to its place "n times" and each "time" requires scanning "n" elements: the result is $n \cdot n = n^2$.

The algorithm has time complexity of $O(n^2)$.

### What happens when we have more than one big-O?

There are times when we have code that looks like the following:

```{src='computer_science/bigo/bigo3' caption='A more complex algorithm to estimate'}
```

As we can see the first part is the bubble sort algorithm, followed by iterating through the (now ordered) list, to print its values.

We can calculate the total estimate as $O(n^2) + O(n)$ and that would be absolutely correct, but as the list grows, the growth rate of $O(n)$ is very minor if compared to $O(n^2)$, as can be seen from the following figure:

![O(n) growth rate, compared to O(n²)](./images/computer_science/o_n_vs_o_n2.svg){width=60%}

So we can drop the $O(n)$ and consider the entire algorithm as an $O(n^2)$ algorithm in its entirety: this means that when dealing with complexity estimates, you always keep the terms that have the largest "growth rate" (check the [Big-O estimates comparison](#big_o_comp) section for more details).

### What do we do with recursive algorithms?

When recursive algorithms are involved, things get a lot more complex, and they involve building recursion trees and sometimes you'll have to use the so-called "master theorem for divide-and-conquer recurrences".

Such methods are outside the scope of this book as of now.

### How do big-O estimates compare to each other? {#big_o_comp}

Here we can see how big-O estimates compare to each other, graphically and how important it is to write not-inefficient algorithms.

![Big-O Estimates, plotted](./images/computer_science/big_o_plot.svg){width=60%}

There is a very specific reason why the $O(2^n)$ estimate is missing from the previous plot: we wouldn't be able to see anything worthwhile if it was included, as seen from the following plot:

![How O(2^n^) overpowers lower complexities](./images/computer_science/big_o_plot2.svg){width=60%}

{{placeholder}}

<!-- TODO: Teach people how to estimate their algorithms -->
