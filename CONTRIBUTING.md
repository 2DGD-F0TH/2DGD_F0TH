Want to contribute? Thank you!
------------------------------

### How to contribute

If you want to contribute to this project, you can follow this procedure.

1) Fork this repository to your own account
2) Check out the *develop* branch with `git checkout develop`
3) Create a new branch with a descriptive name, for instance: `git checkout -b fixes/procedural`
4) Write your contribution
5) Create a Pull Request on the main repository.

### Requirements

There are some requirements the contributions need to meet to be added to the book.

- Algorithms **must** be written in pseudo-code first, in a platform agnostic way. (If you want to put other type of code, like C++ or Python, you can, but there must be a pseudo-code version first), unless they are meant to just state an example and are not meant to be applied in a real-life scenario.
- All algorithms **must** have at least a big-O estimate of their worst case, other estimates in addition to this one are welcome.
- You should avoid using overly technical terms, if possible. If that is not possible, then you should define such term in the "Glossary" appendix of the book and add a ~[g]~ at the end of the term (so that the reader knows such term is in the glossary).

These are not strict requirements, as in they won't make your pull request get rejected, but you may be asked to fix some things.

### Pictures

The pictures used should be simple to understand and not covered by copyright (please no screenshots or real games), where possible images should be saved in the following formats:

- **Diagrams:** Vector PDF format - Some people may find useful to zoom into an image to better understand it and they usually have a low size;
- **Picture Examples:** PNG Format, they have a good quality/compression tradeoff.

Pictures *may or may not* have a caption to describe them, that's up to you.

### Markdown Vs. LaTeX

The book should be written as much as possible in [Pandoc Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), because everyone should be able to contribute to this book easily.

If the situation calls for it (for instance you *absolutely need* a certain type of table) you can use pure LaTeX inline, inside the markdown document.
