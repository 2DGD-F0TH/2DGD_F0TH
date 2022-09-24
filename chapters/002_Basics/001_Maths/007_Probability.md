Basics of Probability
---------------------

Games can make heavy use of probability: for instance when spawning items and treasures. Having a basic grasp of how probability works can make things a lot easier.

### A simple definition of probability

We will define the probability of an event $A$ with a fraction:

$$
P(A) = \frac{The\ outcome\ is\ A}{All\ Outcomes}
$$

For instance: let's take a coin. We want to calculate the probability that a coin toss ends with "head": first we count how many outcomes are possible. Since a coin can land on "tails" or "heads", we have 2 possible outcomes, and head is only one of them.

Thus:

$$
P(heads) = \frac{1}{2} = 0.5
$$

This result can be converted to a percentage, by multiplying it by 100. That means that there's a 50\% chance that a coin toss ends in "heads", shocking, I know.

What if we wanted to know the probability of a coin "not landing on heads"?

Here's a useful formula:

$$
P(\overline{A}) = 1 - P(A)
$$

Thus, by applying such formula on our coin example we have:

$$
P(\overline{Heads}) = 1 - P(A) = 1 - \frac{1}{2} = \frac{1}{2} = P(Tails)
$$

Perfect. Everything as expected.

### Probability of independent events

But what if we wanted to calculate the probability of more than one event?

If our events are independent (that means that the result of one doesn't affect the result of others), we can use the following formula:

$$
P(A\ and\ B) = P(A) \cdot P(B)
$$

Let's return to our coin example: if we wanted to know the probability of two coin tosses landing both on heads, we would have:

$$
P(Heads\ and\ Heads) = P(Heads) \cdot P(Heads) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}
$$

Let's demonstrate that intuitively: since the example is simple, we can literally count the possible outcomes:

| First Toss | Second Toss |
| :--------: | :---------: |
| Heads      | Heads       |
| Heads      | Tails       |
| Tails      | Heads       |
| Tails      | Tails       |

Table: Counting the possible outcomes of two coin tosses

Now we know that there are 4 possible outcomes, and the "Heads + Heads" is only one of them. This confirms our formula.

:::: pitfall ::::
Someone may argue that the probability of "Heads + Tails" is $\frac{1}{2}$, but that would not be correct. We are still strictly tied to the events, that means that "Heads + Tails" means the event of "First toss is heads, second toss is tails", which is different from "Tails + Heads" (first toss is tails, second is heads).
:::::::::::::::::

### Probability of mutually exclusive events

In case the events are mutually exclusive (that means, if one event happens, none of the others can happen), the following formula may be helpful in some occasions:

$$
P(A\ or\ B) = P(A) + P(B)
$$

Going back to our coin example: the probability of a coin toss being "either heads or tails" is $\frac{1}{2} + \frac{1}{2} = 1$.

Another example could be done using a 6-sided dice: each face can be on top with a probability of $\frac{1}{6}$. Let's calculate the probability of either 1 or 6 being face up:

$$
P(1\ or\ 6) = P(1) + P(6) = \frac{1}{6} + \frac{1}{6} = \frac{2}{6} = \frac{1}{3}
$$

### Uniform Distributions

In most cases, we will speak in terms of "uniform distributions", that means that we will be operating on a system where all outcomes have the same probability of happening.

That means that all dices are "fair", all coins are "fair" and all our "bingo bags" have only one instance of a certain number, all of the same size, shape and feel (thus making it impossible for a number to appear more often than any other).

In the grand scheme of things, we are assuming that the `random()` function of our programming language is a uniform distribution, where any number may come out with the same probability of any other.

### How probability is used in games

You can use probability to govern how items spawn: surely you want more precious items to spawn more rarely (with less probability), while more common items should spawn more often.

Let's say we want an item to spawn with 20\% probability: how can we do it?

20\% probability can be rewritten as the decimal $0.2$, such decimal can be obtained with the fraction $\frac{1}{5}$. We have practically solved the problem: we decide on one number between 1 and 5 (inclusive) and we will know that such number will be "extracted" 20\% of the time.

```{src=maths/probability_20 caption="1 (out of 5) will be extracted with about 20% probability"}
```

We will obtain the following result.

![Running the probability_20 example shows the probability floating around 20\%](./images/maths/probability_20.png){width=40%}

But what if we wanted to be a lot more precise? Let's say we want to spawn an item with 13\% probability, how would we go at it?

It's actually pretty simple: out 13\% probability can be represented by the fraction $\frac{13}{100}$. Each number between 1 and 100 (inclusive) has a $\frac{1}{100}$ chance of being extracted. Since extracting one number bars any other number to appear in that extraction we can use the "mutually exclusive events" formula.

$$
P(1\ or\ 2\ or\ ...\ or\ 13) = P(1) + P(2) + ... + P(13) = \frac{1}{100} + \frac{1}{100} + ... + \frac{1}{100} = \frac{13}{100}
$$

:::: tip ::::
If the example is not 100% clear yet, try reading the previous formula right-to-left. That may help.
:::::::::::::

This means that the event "a number between 1 and 13 appears" has a 13\% probability of appearing. We can simplify that statement with "a number less or equal than 13". We can experiment that easily with the following code:

```{src=maths/probability_le_13 caption="A number less or equal than 13 (out of 100) has 13% probability of appearing"}
```

![Running the probability_le_13 example shows the probability floating around 13\%](./images/maths/probability_le_13.png){width=40%}
