Algorithms
----------

When you talk about computer science, you always hear about algorithms: what is an algorithm?

An algorithm can be informally defined as a finite sequence (as in "not infinite") of instructions that are followed to solve certain problems.

There are numerous examples of algorithms, among them we can find:

- Finding the Greatest Common Divisor (GCD) of two numbers;
- Finding the largest number in a list;
- Calculating the $n$th Fibonacci number;
- ...

Algorithms are usually represented in flow charts, or it's more modern counterpart: the UML activity diagram. Sometimes algorithms can be represented in "plain language" (in that case we may end up talking about "pseudocode") or in a programming language.

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
