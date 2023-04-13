### Layering and graphics

There are some things that should be kept in mind when drawing layers for your game, here we talk about some key points about layering and graphics.

#### Detail attracts attention

When it comes to games, it's easy to get too excited and craft your work with the highest amount of detail possible, but there is a problem: detail tends to attract players' attention.

If you put too much detail in the background, you're going to distract them from the main gameplay that happens in the foreground, which can prove dangerous: the graphics can get messy and you can even get to the point of not being able to distinguish platforms from the background.

So a golden rule could be:

> Use high detail in the foreground, gameplay-heavy elements - use less detail in the backgrounds

A good idea to make the background a bit less detailed is using blurring, which allows to keep the overall aesthetic but makes it look "less interesting" than what's in the foreground.

This doesn't mean the background should be devoid of detail, just don't overdo it.

#### Use saturation to separate layers further

Bright colors attract attention as much as detail does, so a good idea is making each background layer more "muted" (color-wise) than the ones in foreground.

The main technique to make backgrounds more muted is lowering saturation, blending the colors with grey: this will make the background less distracting to the player.

So another rule can be written as:

> Layers farther away should have lower color saturation than the layers closer to the camera

#### Movement is yet another distraction

As detail and saturation are due to attract attention from the player, movement is another one of those "eye-catchers" that can make the gameplay more chaotic and difficult for the player.

Small amounts of movement are OK, but fully-fledged animations in the background will prove distracting.

Let's take note of rule number 3 then:

> Try to avoid movement in the background

#### Use contrast to your advantage {#contrast_to_your_advantage}

Complementary colors tend to attract a lot of attention in the points of intersection of their hues.

If backgrounds feature complementary colors, it may distract the player from the main gameplay.

![Which spaceship is easier to spot at a glance?](./images/resources/contrast.png){width=40%}

Our rule number four should then be:

> Keep backgrounds low-contrast to avoid distracting players

Also the opposite rule may apply:

> Keep the main gameplay elements contrasting, so to attract the attention towards them

An orange-robed character will be easier to follow on a blue-ish background, for instance.

#### Find exceptions

Nothing is ever set in stone, and no rules should keep you from playing a bit outside the box and fiddling with some exceptions to a couple rules.

#### Summarizing Layering

Let's take the following image:

![How contrast and detail can help distinguishing foreground and background [^layering_img]](./images/resources/layering/Full.jpg){width=50%}

You can notice that the grass in the foreground has a lot more hues of gold and brown, the trees and the grass are darker and more saturated compared to the background, which is more "muted".

If we were to break down the image into its main layers, from furthest to nearest, we would obtain something like this:

![Breaking down the image allows us to see the differences between the layers [^layering_img]](./images/resources/layering/breakdown.png){width=100%}

From left to right we have:

- **the background:** our sky box, there is very little detail here, just enough to blend the color bands together to make a cohesive piece;
- **the farthest layer:** the trees have very few hues in their bark and leaves, little detail is added, just enough to make out lights and shadows;
- **light rays:** this semi-transparent layer is extremely simple, but being so light and monochromatic it adds variety to make the image interesting without distracting from the foreground too much;
- **the foreground:** this layer features the highest amount of detail, the most evident is the grass (but if you look closely, you may notice some texture in the tree bark too), as well as the most saturated and darkest colors in the picture. The contrast with the other layers makes the foreground "pop up".

To summarize, we can make a handy diagram that will give us a "rule of thumb" when drawing layers for our game:

![A diagram to show how each section affects our perception of a layer](./images/resources/layering_diagram.svg){width=60%}

In short, the closer an object is:

- the more saturated its colors;
- the darker it looks;
- the more detail it has;
- the more it is animated;

[^layering_img]: Image by Roe61 (<https://linktr.ee/Roe61>) used with explicit permission
