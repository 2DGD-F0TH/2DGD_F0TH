Basics of Probability
---------------------

Games can make heavy use of probability: for instance when spawning items and treasures. Having a basic grasp of how probability works can make things a lot easier.

### A simple definition of probability

We will define the probability of an event $A$ with a fraction:

$$
P(A) = \frac{The\ outcome\ is\ A}{All\ Outcomes}
$$

The numerator is called "event space", while the denominator is called "sample space".

For instance: let's take a coin. We want to calculate the probability that a coin toss ends with "head": first we count how many outcomes are possible. Since a coin can land on "tails" or "heads", we have 2 possible outcomes, and head is only one of them.

:::: centering ::::
![](./images/maths/coin_h_t.svg){width=20%}
:::::::::::::::::::

For practicality, we will call "Heads" $H$ and "Tails" $T$.

Thus:

$$
P(H) = \frac{1}{2} = 0.5
$$

This result can be converted to a percentage, by multiplying it by 100. That means that there's a 50\% chance that a coin toss ends in "heads", shocking, I know.

What if we wanted to know the probability of a coin "not landing on heads"?

Here's a useful formula:

$$
P(\overline{A}) = 1 - P(A)
$$

Thus, by applying such formula on our coin example we have:

$$
P(\overline{H}) = 1 - P(A) = 1 - \frac{1}{2} = \frac{1}{2} = P(T)
$$

Perfect. Everything as expected.

### Probability of independent events

But what if we wanted to calculate the probability of more than one event?

If our events are independent (that means that the result of one doesn't affect the result of others), we can use the following formula:

$$
P(A\ and\ B) = P(A \cap B) = P(A) \cdot P(B)
$$

Let's return to our coin example.

:::: centering ::::
![](./images/maths/coin_h_h.svg){width=20%}
:::::::::::::::::::

If we wanted to know the probability of two coin tosses landing both on heads, we would have:

$$
P(H\ and\ H) = P(H) \cdot P(H) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}
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

In the exact same way, we can calculate the probability of a "Heads + Tails" result:

$$
P(H\ and\ T) = P(H) \cdot P(T) = \frac{1}{2} \cdot \frac{1}{2} = \frac{1}{4}
$$

And the previous table confirms our calculations.

:::: pitfall ::::
Someone may argue that the probability of "Heads + Tails" is $\frac{1}{2}$, but that would not be correct. We are still strictly tied to the events, that means that "Heads + Tails" (First toss is heads, second toss is tails), is different from "Tails + Heads" (first toss is tails, second is heads).
:::::::::::::::::

### Probability of mutually exclusive events

In case the events are mutually exclusive (that means, if one event happens, none of the others can happen), the following formula may be helpful in some occasions:

$$
P(A\ or\ B) = P(A \cup B) = P(A) + P(B)
$$

Going back to our coin example: the probability of a coin toss being "either heads or tails" is $\frac{1}{2} + \frac{1}{2} = 1$.

Another example could be done using a 6-sided dice.

:::: centering ::::
![](./images/maths/dice.svg){width=10%}
:::::::::::::::::::

Each face can be on top with a probability of $\frac{1}{6}$. Let's calculate the probability of either 1 or 6 being face up:

$$
P(1\ or\ 6) = P(1) + P(6) = \frac{1}{6} + \frac{1}{6} = \frac{2}{6} = \frac{1}{3}
$$

:::: note ::::
Considering the latest "tossing two coins" example, we can calculate the probability of "one coin lands on heads and the other lands on tails" with the previous formulas, since coin tosses tick both the "independence" and "mutual exclusivity" boxes.

$$
P((H\ and\ T)\ or\ (T\ and\ H)) = P(H\ and\ T) + P(T\ and\ H) = \frac{1}{4} + \frac{1}{4} = \frac{1}{2}
$$
::::::::::::::

### Probability of non-mutually exclusive events

Not all events are mutually exclusive. Let's think, for instance, about a deck of cards: what if you wanted to know the probability of drawing either a card of hearts or a face card (Jack, Queen or King)?

:::: centering ::::
![](./images/maths/cards.svg){width=20%}
:::::::::::::::::::

We need to use a different formula in that case, which is the following one:

$$
P(A\ or\ B) = P(A \cup B) = P(A) + P(B) - P(A\ and\ B)
$$

:::: note ::::
Why are we subtracting $P(A\ and\ B)$?
Because if we didn't, we would be counting the face cards of hearts twice: once when we count the card of hearts, and once when we count all the face cards.
::::::::::::::

Let's continue with our example.

A standard deck has 52 cards, 13 for each seed. This means we would have 13 cards of hearts: $P(A) = \frac{13}{52}$.

The same deck of cards also has 3 "face cards" for each seed, totalling 12: $P(B) = \frac{12}{52}$.

Since there are face cards of the hearts seed, we need to account for those too, totalling 3: $P(A\ and\ B)= \frac{3}{52}$.

This means that the probability we're looking for is calculated as follows:

$$
P(A\ or B) = P(A) + P(B) - P(A\ and\ B) = \frac{13}{52} + \frac{12}{52} - \frac{3}{52} = \frac{22}{52} = \frac{11}{26}
$$

### Conditional Probability

:::: wizardry ::::
Conditional probability doesn't have a lot of uses in game development, but it's worth mentioning it if you want to have a probabilistic approach to AI. Feel free to quickly skim through this section.
::::::::::::::::::

Sometimes you may need to consider the probability of a certain event, given that another event happens. This is called "conditional probability", and it can be calculated as follows:

$$
P(A|B) = \frac{P(A\ and\ B)}{P(B)} = \frac{P(A \cap B)}{P(B)}
$$

Conditional probability can be used to enrich the decision making used in enemy AI, for instance.

Let's take a concrete example, taken straight from the famous tabletop RPG Dungeons&Dragons, and see how probability can be applied to decision making.

> You're fighting against an enemy. Both you and the enemy are close to fatal damage: you have 1HP, while the enemy has 3HP left.
>
> To attack an enemy you need to roll a 20-sided dice (called a d20): if the number rolled is 13 or higher you will hit, else you will miss.
>
> If you hit, you will roll a 6-sided dice (called a d6): the number rolled will decide how much damage you will deal, so you need 3 or more.
>
> We need to find the probability of killing the enemy within the next turn to decide our next move.

First of all, let's name the events:

- **H** Will be the event "hit", which means that the d20 rolled a number that is 13 or higher.
- **F** Will be the event "fatal damage", which means that the d6 rolled a number that is 3 or higher.

Now we will calculate the probabilities we need for our calculation:

$$
P(H) = \frac{8}{20} = \frac{2}{5}
$$
$$
P(F) = \frac{4}{6} = \frac{2}{3}
$$

Our objective is calculating "the probability of doing at least 3HP of damage, given that we hit the enemy". This is represented as:

$$
P(F|H) = \frac{P(F \cap H)}{P(H)}
$$

This means we will have to calculate another probability, which is quite easy:

$$
P(F \cap H) = P(F) \cdot P(H) = \frac{2}{3} \cdot \frac{2}{5} = \frac{4}{15}
$$

Now we are ready to calculate everything we need:

$$
P(F|H) = \frac{P(F \cap H)}{P(H)} = \frac{\frac{4}{15}}{\frac{2}{5}} = \frac{4}{15} \cdot \frac{5}{2} = \frac{2}{3}
$$

Given a 66\% chance of success, you may decide that attacking is worth the risk. Such decision may be hard-coded into an AI, for instance if the probability is higher than 50\% the AI may choose to attack instead of retreating and call for backup.

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
If the example is not 100\% clear yet, try reading the previous formula right-to-left. That may help.
:::::::::::::

This means that the event "a number between 1 and 13 appears" has a 13\% probability of appearing. We can simplify that statement with "a number less or equal than 13". We can experiment that easily with the following code:

```{src=maths/probability_le_13 caption="A number less or equal than 13 (out of 100) has 13% probability of appearing"}
```

![Running the probability_le_13 example shows the probability floating around 13\%](./images/maths/probability_le_13.png){width=40%}

:::: tip ::::
You can extend the example above to fractions of a percentage by using bigger numbers: if you wanted a 13.5% probability, you would use all numbers less than or equal to 135, out of 1000.
:::::::::::::

### Tiered Prize Pools

We can use what we learned with probability to create a tiered prize pool. For instance we decide that killing a certain enemy will always drop something, the tier of such item is according to the follow probability list:

- 50\% probability for a common item to drop (for instance a scrap of leather);
- 30\% probability for an uncommon item to drop (like a lower-grade potion);
- 15\% probability for a rare item to drop (a good sword, for instance);
- 5\% probability for an epic item to drop (a unique armor, for example);

![Intuitive representation of our prize pool](./images/maths/probability_prize_pool.svg){width=40%}

:::: longdesc ::::
A bar showing the distribution of prize categories. The bar starts from 0 and ends at 100. The bar is divided in 4 colours: gray (common items) takes up 50% of the total space, green (uncommon items) takes up 30% of the total space, blue (rare items) takes up 15% of the total space, while orange (epic items) takes up the remaining 5%.
::::::::::::::::::

In that case we can chain ifs to bring our tiered prize pool to life:

```{src=maths/probability_tiered_pool caption="How to implement a tiered prize pool selector"}
```

#### Introducing a "luck" stat

In many RPGs there is a "luck" statistic that affects how item drops happen, in that case we will need to change how tiered prize pools are given out. Things can get complicated quite quickly.

Let's imagine a simple situation: one point of "luck" gives a $1\%$ probability of getting an item of each tier higher than "Common", while at the same time reducing the probability of finding a "common" item.

At a first glance, it seems simple: take each "non-common" class and "add 1", then take the "common" class and "remove 1 for each point given". But what would happen if the luck stat is higher than the probability of a "common" item? It should probably start taking away probability from "uncommon" items to give out "rare" and "epic" items.

Let's see a possible implementation:

```{src=maths/probability_luck caption="A possible implementation of a luck stat"}
```
