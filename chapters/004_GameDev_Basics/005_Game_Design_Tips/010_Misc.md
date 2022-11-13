Miscellaneous
-------------

This section denotes some various things that don't really fit in "tips and tricks" but are still related to game design.

### You cannot use the "Red Cross" in games

::: note :::
What follows **is not legal advice**. I am not a lawyer.

If you want to know more (as in quantity and quality of information), contact your favourite lawyer.
:::::::::::::::

When you are developing a game, you will be tempted to use the famous "red cross" symbol on your health packs or health-related items.

**Don't do that**

The red cross symbol (a red cross over white background) is not in the public domain, but it's actually a symbol governed by the ICRC (International Committee of the Red Cross) and you may get in legal trouble for misusing it.

Halo and Doom changed their health packs symbol, from the red cross to a "red H" and a "pill" respectively.

The ICRC enforcement of this rule is inconsistent, but it would technically be a violation of the First Geneva Convention, chapter VII, articles 44 and 53.

Also states themselves (like Canada) tend to have rules of law regulating the use of the symbol in more detail.

### Auto-saving

Some people may consider auto-saving a simple "quality of life improvement", but it can also save the players a lot of frustration in case your game crashes: trust me, no matter how good your programming is, your game will crash (it may be a buggy graphics driver, an edge case that hits 0.0001% of the time or just bad luck).

If possible, you should provide the player with both an auto-save feature and a "manual save" one, this way the player can save where they want but also have a back-up just in case.

To implement an auto-saving feature, we need a slot to auto-save into, so we can choose one of two ways:

- **Choosing the save slot when starting a new game:** this means that the auto-save feature will auto-save and overwrite the selected save slot at every major event, which may be not desired. This is where the manual saving feature comes handy: allowing the player to save manually will also allow them to create a backup savefile.
- **Dedicated "auto-save" slot:** this leaves the manual saving feature intact, but also adds a "special saving slot" the player can't save onto. This slot is dedicated to the most recent auto-save (regardless of the save slot we load from).

### Feedback is important

It is extremely important to add feedback to actions, such as hits: a good visual feedback and the right sound can make all the difference in the world for your game.

The most common visual reaction to a hit is lighting up (by adding a white overlay) the sprite that got hit: this way it is really evident that a hit happened.

The visual feedback should also mirror the effectiveness of the hit too. An explosive weapon should do tons more damage than a single bullet: if this doesn't happen the weapons will feel unbalanced and just badly designed.
