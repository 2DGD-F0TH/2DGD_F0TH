Economy
-------

Some games (not only MMOs) feature an "economy" side to their gameplay: this can prove to be something really difficult to balance without creating a virtual financial disaster.

This section will give you some basics to get things right.

### Supply and Demand

Every economy is (at least in part) governed by the laws of supply and demand, which can be graphically represented in the following graph:

![A simplified vision of supply and demand](./images/balancing/supply_demand.svg){width=50%}

:::: note ::::
This is an oversimplification of how the market (and the economy in general) works, just enough to keep you far away from the most common issues.
::::::::::::::

We can get some takeaways from such graph:

- If the demand is low (noone wants the product), suppliers will try to "boost it" by lowering prices;
- If the demand is high (many want the product), suppliers will try to earn more by boosting prices;
- If the supply is low (the product is rare), people will value it more (the price will be higher);
- If the supply is high (the product is really common), people will value it less (paying it less).

This also shows that artificially keeping the supply low will make the product feel more valuable, allowing to ask for higher prices.

Another thing to remember: money is a good too, and is subject to the same laws.

### Money sources and sinks

Any artificial economy is usually composed by 2 "components":

- **Money Sources:** they create money from nothing, these can be quest givers, treasure chests and the like;
- **Money Sinks:** places that "destroy money", these are NPC salesmen at the market (that create items from nothing), fortune machines, anything that takes or exchanges money for something else.

Sources and sinks are extremely important and should be carefully balanced, since an imbalance in the quantity of money created and destroyed can have catastrophic effects. Among those, uncontrolled inflation and deflation are the most prominent.

### Inflation

Inflation is a phenomenon where prices usually rise uncontrollably: this means that money "lost its value".

This is usually due to the massive presence of money in the economy, so in a source/sink view, the money sources emit much more money than what the sinks can consume.

As a consequence fixed-price operations (like if you put "repair a weapon" at a fixed 50 golds) become incredibly cheap, while products in the market become prohibitly expensive.

In a supply/demand perspective, there is big supply of money which triggers little demand for it (since it's so common), while there is a big demand for products (thus raising the prices).

This may end with people having loads of money and noone accepting them for trades. Bartering may arise as an alternative to money.

### Deflation

Deflation is a phenomenon where prices usually have a drop: this means that money has "too much value".

This has the exact opposite causes of inflation: there is too little money in the economy, so the money sources don't emit enough money and there are too many sinks that can consume it.

As a consequence fixed-price operations become extremely expensive (if you have 100 gold, paying 50 gold to repair a weapon may seem a lot), while products in the market become extremely cheap.

Again, in a supply/demand perspective, there is a low supply of money (making it more valuable), while demand is really high.

This can trigger "money hoarding" thus freezing the economy, sometimes bartering can arise as an alternative way to exchange goods without involving the "precious precious money". Some operations that require a minimum amount of money may even get locked because of the little amount of money circulating.
