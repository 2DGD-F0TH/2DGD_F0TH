Fonts
-----

### Font Categories

Before starting with fonts and the ways they can be integrated in your game, we should start with some definitions and categorizing fonts by their own characteristics.

#### Serif and Sans-Serif fonts

In typography, *serifs* are small strokes that get attached to larger strokes in a letter (or symbol) of certain fonts. The font families that make use of serifs are called **serif fonts** or **serif typefaces**.

Serif fonts look more elegant and give a "roman" feeling (in fact, serif fonts are also called *roman* typefaces) and are good for games that take place in historical settings or need a semblance of pretend "historical importance" (in their own world).

![Example of a serif font (DejaVu Serif)](./images/resources/serif_font.png){width=30%}

Serif fonts look better on paper and could come out as a bit harder to read on screens. A famous serif font is Times New Roman.

On the opposite side, we have **sans-serif fonts**, where such small strokes are absent. Sans-Serif fonts seem easier to read on screens and look simpler, but they don't look as good on paper, when long text bodies are involved.

![Example of a sans-serif font (DejaVu Sans)](./images/resources/sans_serif_font.png){width=30%}

A famous sans-serif font is Arial.

#### Proportional and Monospaced fonts

The majority of fonts used today are **proportional**, where each letter occupies its own space proportional to its own width. Examples of proportional fonts are Times New Roman and Arial.

![Example of a proportional font (DejaVu Serif)](./images/resources/proportional_font.png){width=30%}

Notice the difference in width between certain pairs of letters, like "i" and "o" or "a" and "l".

Proportional fonts are good for general text that don't have any particular constraint.

On the opposite side, there are **monospaced** fonts, also called **fixed-width** fonts. In these font families, each letter occupies the same amount pre-defined width.

![Example of a monospaced font (Inconsolata)](./images/resources/monospace_font.png){width=30%}

Again, notice how all letters occupy the same horizontal space.

Monospaced fonts are used for computer texts, coding and ascii-art. Examples of monospaced fonts are Courier New and Inconsolata.

### Using textures to make text

If you want to make text to show on the screen, an idea could be creating a sprite sheet that contains all the characters that you want to use, then you can split it at runtime and (knowing that each letter is in a grid) you can index each letter by its position and write text.

Let's imagine the simplest case: we need to use only uppercase letters, and our sprite sheet is 1 row by 26 columns:

![A simple spritesheet for rendering text using textures](./images/resources/texture_text.svg){width=90%}

That looks awfully similar to an array, doesn't it? We just need to know how big each tile is (and the easiest way to do so it making them all the same size) and their index in the array, connect them to a letter and we can make text!

![Indexing our spritesheet for rendering](./images/resources/texture_text_index.svg){width=40%}

So, if we were to write the word "HELLO", we would need the letters at index 7, 4, 11 (twice) and 14. If the tiles were 32 x 32 pixels, we would need a surface that is 32 pixels tall and 160 pixels wide.

The code to make text from a texture could look something like this:

```{src='resources/texture_text' caption='A simple algorithm to create a text using a texture'}
```

And that's how you get a text from a texture. Pun (maybe) not intended.

This is a really quick way to render some text, and it has some great advantages: the characters can be as complex as you want, your spritesheet contains only the characters that you need (and not anything else), for instance.

But with those advantages, you can have some disadvantages: scaling and resizing can be a true issue. You are not rendering any "vector graphics" (like real fonts do), so if you have to scale up the text, it may result in fuzziness or pixellated look.

### Using Fonts to make text

{{placeholder}}

<!-- TODO: Good and resizes well, but can be slower cause the text needs to be rendered -->

