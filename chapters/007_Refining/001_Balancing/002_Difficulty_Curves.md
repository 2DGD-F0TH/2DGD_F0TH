Difficulty curves {#diff_curves}
-----------------

When designing our game, it may be useful (sometimes mandatory) to have a high-level view of how our game's difficulty will evolve as the game itself is played. If we take a Cartesian plane and define time as the $x$ axis, while the "perceived difficulty" is plotted on the $y$ axis, we would obtain a **difficulty curve**, a high-level representation of how difficulty evolves as the game is played.

Knowing some basic difficulty curves, as well as their pros and cons, may give you an idea of how you want to build and balance your game. This section will be heavy on charts, so be prepared!

### Simple Lines

Let's start with simple lines, they can be straight lines or simple curves that don't feature any waviness or wobbliness. These are usually the simplest to learn but that doesn't mean they are free from complicated drawbacks. Let's check some out.

### Flat Line

The first curve is the "flat line", which is a simple horizontal line that spans the whole playtime. It can't get simpler than that.

![A Flat line difficulty curve](./images/balancing/flat_line.svg){width=40%}

When the player selects a difficulty level, the difficulty stays around that value (with no real perceivable change) for the entire playthrough. That means that there is no "evolution" to take care of and no long-term balancing to perform.

This curve also represents a way of balancing your game that gets boring rather quickly, since the player gets better at the game as time passes, but the game doesn't "follow them" by giving them a higher challenge.

#### Linear Increase

To solve the issue of the flat line, you can add a linear increase to your difficulty in an effort to "keep up" with the player.

![A linearly increasing difficulty curve](./images/balancing/increasing_line.svg){width=40%}

This curve is usually easy to manage, giving a lot of control over the initial difficulty and its evolution. The player is challenged for longer periods of time, since the game becomes more difficult the further the player plays it.

The biggest drawback of this kind of curve is its predictability: after a while, a somewhat "expert" player can predict "by feel" the upcoming challenges and prepare as a consequence, thus "squashing" the final part of the curve.

![As the player learns to predict, the difficulty curve changes from our design](./images/balancing/increasing_line_squashed.svg){width=40%}

#### Logarithmic Line

The logarithmic line is usually presented as a "guide" for more advanced types of curves. This is due to the fact that it has some major issues.

![A Logarithmic difficulty curve](./images/balancing/logarithmic_line.svg){width=40%}

The beginning of the game has a steep learning curve, which eases up as the game goes on. This means that the game is really hard at the beginning but the challenge dies down towards the end, which can make for a game very difficult to learn but not much more.

This can be a good curve if you want to "test the might and patience" of your players, but if not paired with a different approach in the late game, it may end up being boring in the long run.

#### Exponential Line

The complete opposite of the logarithmic line is the exponential one. This has a lot more use in game design that the previous example.

![An exponential difficulty curve](./images/balancing/exponential_line.svg){width=40%}

The exponential difficulty curve gives the player a very relaxed beginning as well as a late game that can get really hard really fast (to the point that it can be too hard). This curve is the literal definition of a game that is "easy to learn but hard to master".

### Wave patterns

The difficulty curves that we've seen so far have all one thing in common: they are simple and feature no real "lack of predictability", which can make a game a bit boring in the long run. Not because it's not challenging, but because it's predictable.

#### Linearly Increasing wave

Adding some waviness to the linearly increasing line can add some spice to the game very easily.

![A linearly increasing wavy difficulty curve](./images/balancing/increasing_wave.svg){width=40%}

This is a very efficient way of working, since it makes things more interesting, but if not implemented correctly it can lead to very high difficulty during the late game, since the "wave" may compound with an already high difficulty level.

#### Logarithmically Increasing wave

To try and fix the issues from the linearly increasing wave pattern, we may want to tie our difficulty to a logarithmic line.

![A Logarithmically increasing wavy difficulty curve](./images/balancing/logarithmic_wave.svg){width=40%}

This kind of difficulty curve tends to "squash" the challenge towards the mid-to-late game, thus making the game a bit less difficult if the "wave" compounds with an already high difficulty level.

As a drawback, this curve may feel more "predictable" towards the late game, since the difficulty tends to get very "horizontal" towards the end; the wavy pattern helps keeping the predictability at bay, thus lengthening the enjoyability of the game.

### Interval Patterns

For games that involve some random generation, like roguelites, we may want to "clamp" the difficulty between a "minimum" and a "maximum" but still allow for "runs" that feel different in difficulty from each other.

In this section we will show only wavy patterns, to exemplify the most "difficult to design" patterns, butt all patterns apply to simple lines too.

#### Simple Interval

The simplest way to implement an interval pattern is just defining a minimum and a maximum difficulty and setting the difficulty in such interval.

![A simple wavy difficulty interval](./images/balancing/simple_interval.svg){width=40%}

This pattern is good for unpredictable challenges, but it is so unpredictable that you have no control over the initial difficulty either. This means that you may have a run of your game starting way too hard, while the next one may end up being very easy.

#### Widening Interval

To solve the lack of control over the initial difficulty, you may want to shape your interval like a letter "V" (just on its side).

![A widening and wavy difficulty interval](./images/balancing/widening_interval.svg){width=40%}

The widening interval allows you to have almost total control over the initial difficulty, while still keeping an unpredictable challenge in the mid and late game. The fact that the pattern widens towards the late game may end up being a drawback in some situations, since the game may have a really easy or really hard "ending". This makes for a sometimes inconsistent experience.

#### Widening Interval with Logarithmic trend

When things tend to get out of control towards the late game, logarithmic curves come to our rescue and this is one of those times.

![A widening wavy difficulty interval with a logarithmic trend](./images/balancing/widening_interval_log.svg){width=40%}

By tying our widening interval to a logarithmic line we have a way to better control how the game's difficulty evolves in the mid-to-late game. This gives the game's difficulty an "increasing trend" and coupled with a wavy difficulty line it can still be unpredictable enough to be enjoyable.

Such control doesn't come cheap though, since having so many things to control (the initial difficulty level, how much the curve widens, how fast things evolve) can be really difficult.

### This is not everything

The difficulty curves that we've seen so far are definitely not the only ones that exist in video game development. You can mix and match until you reach a result that may look fun (in theory) and appeal to the player base that you have chosen.

Here we take a look at some more elements and curves that don't fit the previous description.

#### Sawtooth pattern

Every time we introduce a new mechanic, it may be useful and fun to let the player make large use of it, thus making the game a bit easier.

![A sawtooth difficulty curve](./images/balancing/sawtooth_line.svg){width=40%}

This gives our curve a sawtooth-like shape, where the game gets slightly easier every time a new mechanic (like a powerup, or a tool) gets introduced, just to climb higher than the previous maximum. This can give the player an idea of "a reward for doing something difficult", and such reward is the new mechanic.

#### What not to do

When designing a video game, there are at least as many things you should as the ones you may want to do. One of the things you shouldn't do at all is adding "difficulty spikes" to your gameplay.

![Difficulty spikes are not good](./images/balancing/difficulty_spike.svg){width=40%}

Difficulty spikes don't look good in graphs and don't make a game challenging or fun, they interrupt the natural flow of the game and end up frustrating the player. This may include an extremely precise jump in a 2D platformer just after a series of simple levels (even worse if such jump if far from the last checkpoint), or a very difficult boss that has no real place being there it is (difficulty-wise).

Another thing that you may want to avoid is making the game easier for experts: this may include adding a secret stash of collectibles (like powerups or skill points) in a place where only expert (or very very good) players can reach.

Furthermore, you should avoid punishing players who "don't play that well" further than the minimum necessary: losing a life is already a strong "punishment", if you make them lose all their gear without possibility of recovery (this goes for skills too), your game will be put on the shelf by the majority of your player base.

Avoid letting players "skip learning skills", since they will find themselves in a world of trouble as soon as such skill is necessary to continue the game. The player will feel lost at first, then think that the game glitched out and only then (if they didn't uninstall the game already) they make backtrack to look for something they missed.

Try to avoid overloading the players with information: when dealing with a tutorial that lasts longer than it should or that presents way too much information at once, players will lose focus and will tend to skip steps just to "get over it".

#### Beyond difficulty

Difficulty is not everything in a game: a game may greatly enjoy from other elements, like comedy or just the feeling of relax that may come from a farming game. Some players really enjoy escaping the hectic city life to lose themselves in the rhythm of nature (although it is a simplified an virtual version of it).

Other games benefit from collectathon traits: deck-building games are a prime example. You start with a basic "deck of cards" which have certain powers, as you play the game more cards unlock and soon enough the player is enjoying the feel of strategy that comes from "building the perfect deck".
