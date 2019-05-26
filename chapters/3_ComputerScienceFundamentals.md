\null\clearpage
Some Computer Science Fundamentals
===================================

In order to understand some of the language that is coming up, it is necessary to learn a bit of the computer science language and fundamentals.

This chapter will briefly explain some of the language and terms used, their meaning and how they contribute to your activity of developing games.

In this chapter we'll assume you already know what the following terms mean:

- Truth Table
- Algorithm

Estimating the order of algorithms
---------------------------------------

Now more than ever, you need to be able to be efficient. How do you know how "efficient" some piece of algorithm is?

Seeing how much time it takes is not an option, computer specifications change from system to system, so we need something that could be considered "cross-platform".

This is where notations come into play.

There are 3 types of Asymptotic notation you should know: $\Omega$, $\Theta$ and O.

**$\Omega$()** represents **a lower bound**: this means that the algorithm will take **at least** as many cycles as specified.

**O()** represents **an upper bound**: it's the most used notation and means that the algorithm will take **at most** as many cycles as specified.

**$\Theta$()** is a **tight bound**, used when the big-O notation and the big-$\Omega$ notation have the same value, which can help define the behaviour of the algorithm better.

We will now talk about the most common Big-O notations, from "most efficient" to "least efficient".

### O(1)

An algorithm that executes in **O(1)** is said to execute "in constant time", which means that no matter how much data is input in the algorithm, said algorithm will execute in the same time.

An example of a simple O(1) algorithm is an algorithm that, given a list of elements (with at least one element), returns `True` if the first element is `null`.

~~~~ {.python .numberlines}
def isFirstElementNull(elements):
    return elements[0] == None
~~~~

To be precise, this algorithm will perform both in O(1) and $\Omega(1)$, so it will perform in $\Theta(1)$.

### O(log(n))

An algorithm that executes in O(log(n)) is said to execute in "logarithmic time", which means that given an input of **n** items, the algorithm will execute **log(n)** cycles at most.

An example of a O(log(n)) algorithm is the so-called "binary search" on a ordered list of items.

~~~~
binarySearch(elements, element_to_find):
    get middle element
    is it the element_to_find?
        yes: return the middle element position
    else:
        is the element to find bigger than the "middle element"?
            yes: perform binarySearch on the half of the list bigger than "middle element"
            no: perform binarySearch on the half of the list smaller than "middle element"
~~~~

The best case is the time when you get the element to find to be the "middle element" of the list, in that case the algorithm will execute in linear time: $\Theta(1)$ - You need **at least one lookup** ($\Omega(1)$) and **at most one lookup** ($O(1)$).

In the worst case, the element is not present in the list, so you have to split the list and find the middle element until you realize that you don't have any more elements to iterate - this translates into a **tight bound** of $\Theta(log_{2}n)$

### O(n)

An algorithm that executes in O(n) is said to execute in "linear time", which means that given an input of **n** items, the algorithm will execute at most **n** cycles.

An example of a simple O(n) algorithm is the one that prints a list, element by element.

~~~~
printList(list):
    for each element in list:
        print element
~~~~

It's evident that this algorithm will call the `print` function `n` times, where `n` is the size of the list. This translates in a $\Theta(n)$ complexity, which is both $O(n)$ and $\Omega(n)$.

There is no "best" or "worst" case here, the algorithm prints `n` elements, no matter their order, the alignment of planets and stars or the permission of its parents.

### O(n·log(n))

An algorithm that executes in O(n·log(n)) executes in a time slightly longer than a linear algorithm, but it's still considered "ideal". These algorithms are said to execute in "quasi-linear", "log-linear", "super-linear" or "linearithmic" time.

Given an input of **n** elements, these algorithms execute **n·log(n)** steps, or cycles.

Some algorithms that run in O(n·log(n)) are:

- Quick Sort
- Heap Sort
- Fast Fourier Transforms (FFT)

These algorithms are more complex than a simple example and would require a chapter on their own, so we'll leave examples aside for now.

### O(n^2^)

Quadratic algorithms, as the algorithms that execute in O(n^2^) are called, are the door to the "danger zone".

These algorithms can eat your CPU time quite quickly, although they can still be used for small computations somewhat efficiently.

Given an input of **n** elements, these algorithms execute **n^2^** cycles, which means that given an input of **20** elements, we'd find ourselves executing **400** cycles.

A simple example of a quadratic algorithm is "bubble sort". A pseudo-code implementation is written here.

~~~~
bubbleSort(A : list of sortable items )
    n = length(A)
    repeat
        swapped = false
        for i = 1 to n-1 inclusive do
            if A[i-1] > A[i] then
                swap( A[i-1], A[i] )
                swapped = true
            end if
        end for
    until not swapped
~~~~

Anything with complexity higher than `O(n^2^)` is usually considered unusable.

### O(2^n^)

Algorithms that execute in exponential time are considered a major code red, an will usually be replaced with heuristic algorithms (which trade some precision for a lower complexity).

Given an input of `n` elements, an algorithm that executes in O(2^n^) will execute 2^20^ = 1 048 576 cycles!


A primer on calculating the order of your algorithms
-----------------------------------------------------

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

\begin{table}[H]
    \centering
    \caption{Karnaugh Map for A XOR B}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & 1 & 0\\
        \hline
    \end{tabular}
\end{table}

Now we have to identify the biggest squares or rectangles that contain 2^n^ elements equal to 1 so that we can cover all the "1" values we have (they can overlap). In this case we're unlucky as we have only two small rectangles that contain one element each:

\begin{table}[H]
    \centering
    \caption{Karnaugh Map where the elements of the two "rectangles" have been marked green and red}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & \textcolor{red}{1} \\
        & \textbf{1} & \textcolor{green}{1} & 0\\
        \hline
    \end{tabular}
\end{table}

In this case, we have the result we want with the following formula: $f = (A \land \bar{B}) \lor (\bar{A} \land B$)

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

\begin{table}[H]
    \centering
    \caption{Karnaugh Map with a "don't care" value}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & 1 & x\\
        \hline
    \end{tabular}
\end{table}

Now we have a value that behaves a bit like a "wild card", that means we can pretend it's either a 0 or 1, depending on the situation. In this example we'll pretend it's a 1, because it's the value that will give us the biggest "rectangles".

\begin{table}[H]
    \centering
    \caption{Karnaugh Map where we pretend the "don't care" value is equal to 1}
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & 1 & 1\\
        \hline
    \end{tabular}
\end{table}

Now we can find two two-elements rectangles in this map.

The first is the following one:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & 1 \\
        & \textbf{1} & \textcolor{red}{1} & \textcolor{red}{1}\\
        \hline
    \end{tabular}
\end{table}

In this case, we can see that the result is 1 when $B=1$, no matter the value of A. We'll keep this in mind.

The second rectangle is:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c|}
        \hline
        & & \multicolumn{2}{c|}{A}\\
        & & \textbf{0} & \textbf{1}\\
        \hline
        \multirow{2}{*}{B} & \textbf{0} & 0 & \textcolor{green}{1} \\
        & \textbf{1} & 1 & \textcolor{green}{1}\\
        \hline
    \end{tabular}
\end{table}

In this case, we can see that the result is 1 when $A=1$, no matter the value of B.

This translates into a formula of: $f= (A) \lor (B)$, considering that we don't care about the result that comes out when $A=1$ and $B=1$.

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

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & x & 1\\
        & \textbf{10} & 0 & 1 & 1 & 1\\
        \hline
    \end{tabular}
\end{table}

We can see two rectangles that contain 2^n^ items, one with 2 items, the other with 8, considering the only "don't care" value as 1.

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & \textcolor{red}{1} & \textcolor{red}{1}\\
        & \textbf{01} & 0 & 0 & \textcolor{red}{1} & \textcolor{red}{1}\\
        & \textbf{11} & 0 & 0 & \textcolor{red}{x} & \textcolor{red}{1}\\
        & \textbf{10} & 0 & 1 & \textcolor{red}{1} & \textcolor{red}{1}\\
        \hline
    \end{tabular}
\end{table}

In this first rectangle, we can see that the values of C and D don't matter towards the result, as well as the value of B. The only variable that gives the result on this rectangle is $A=1$. We'll keep that in mind

Let's see the second rectangle:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & x & 1\\
        & \textbf{10} & 0 & \textcolor{green}{1} & \textcolor{green}{1} & 1\\
        \hline
    \end{tabular}
\end{table}

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

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & 0 & 1\\
        & \textbf{10} & 0 & 1 & 1 & 1\\
        \hline
    \end{tabular}
\end{table}

Find the biggest rectangles:

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & \textcolor{red}{1}\\
        & \textbf{01} & 0 & 0 & 1 & \textcolor{red}{1}\\
        & \textbf{11} & 0 & 0 & 0 & \textcolor{red}{1}\\
        & \textbf{10} & 0 & 1 & 1 & \textcolor{red}{1}\\
        \hline
    \end{tabular}
\end{table}

\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & \textcolor{green}{1} & \textcolor{green}{1}\\
        & \textbf{01} & 0 & 0 & \textcolor{green}{1} & \textcolor{green}{1}\\
        & \textbf{11} & 0 & 0 & 0 & 1\\
        & \textbf{10} & 0 & 1 & 1 & 1\\
        \hline
    \end{tabular}
\end{table}


\begin{table}[H]
    \centering
    \begin{tabular}{|c c|c c c c|}
        \hline
        & & \multicolumn{4}{c|}{AB}\\
        & & \textbf{00} & \textbf{01} & \textbf{11} & \textbf{10}\\
        \hline
        \multirow{4}{*}{CD} & \textbf{00} & 0 & 0 & 1 & 1\\
        & \textbf{01} & 0 & 0 & 1 & 1\\
        & \textbf{11} & 0 & 0 & 0 & 1\\
        & \textbf{10} & 0 & \textcolor{purple}{1} & \textcolor{purple}{1} & 1\\
        \hline
    \end{tabular}
\end{table}

Extract the result: $f = (A \land \bar{C}) \lor (A \land \bar{B}) \lor (B \land C \land \bar{D})$
