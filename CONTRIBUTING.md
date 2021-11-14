Want to contribute? Thank you!
------------------------------

### How to contribute

If you want to contribute to this project, you can follow this procedure.

1) Fork this repository to your own account
2) Check out the *develop* branch with `git checkout develop` or your tool of choice
3) Create a new branch with a descriptive name, for instance: `git checkout -b fixes/procedural`
4) Write your contribution
5) Create a Pull Request on the main repository.

### Contribution Guidelines

There are some guidelines your contributions should meet to be added to the book.

- Algorithms **must** be written in pseudo-code first, in a platform agnostic way: the pseudocode version is the one that all the language-specific versions are based on. If you want to put other type of code, like C++ or Python, you can, in the appropriate "listings" folders but there must be a pseudo-code version.
- The most important algorithms (like search algorithms or graph navigation ones) will greatly benefit from a big-O estimate of their worst case (although this won't be mandatory), other estimates in addition to this one are welcome (slices of code don't necessarily need estimates, but you can add them if you think they would help). This will help the reader distinguish between better and worse-performing algorithms.
- You should avoid using overly technical terms, if possible. If that is not possible, then you should define such term in the "Glossary" appendix of the book and add a ~[g]~ at the end of the term (so that the reader knows such term is in the glossary).
- If you want to add a `{{placeholder}}`, remember to follow that with a `<<!--TODO: ..... -->` so that it's easy to understand what's missing.

These are not strict requirements, they won't make your pull request get rejected, but you may be asked to tweak some things.

### Adding a new programming language

This book is built in a way that allows everyone to have the book in their own favourite programming language, as long as the listings for such language are available. LaTeX will take care of the highlighting.

If you want to add a new programming language, navigate to the `listings` folder, copy the `pseudocode` folder (or any other code folder you want) and name it with your language's name. The languages that feature syntax highlighting are the ones supported by [skylighting](https://github.com/jgm/skylighting) and LaTeX's "listings" package.

The only thing left to do is translating all the pseudo-code listings into your favourite programming language.

After that you can try and make a copy of the book, by editing the `Makefile`. Remember to add the new language in the `README.md` file.

You may also want to refer to the "Dynamic Code Blocks" section of this document.

### Language Addons / Extensions

You can add new listings that include your favourite framework/library by creating a new folder in the language of choice and using the `lang_extension` metavariable (just add `-M lang_extension="your_folder_name"`) to the commands in the MakeFile.

#### What if my programming language is not supported?

No worries, you can still add your own. Check the [FAQ Document](FAQ_GUIDES.md).

### Pictures

The pictures used should be simple to understand and not covered by copyright (screenshots of real games should be avoided if possible, as copyright and the "fair use" doctrine are quite complicated), where possible images should be saved in the following formats:

- **Diagrams:** Vector SVG format (possibly optimized) - Some people may find useful to zoom into an image to better understand it and they usually have a low size;
- **Picture Examples:** PNG Format, they have a good quality/compression tradeoff.

The most important pictures *should* have a caption to describe them, but not necessarily, that's up to you.

Images that are subject to change and corrections (like UML diagrams) should include a raw format inside the `raw_resources/images/` folder.

### Markdown Vs. LaTeX

The book should be written as much as possible in [Pandoc Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), because everyone should be able to contribute to this book easily.

If the situation calls for it (for instance you *absolutely need* a certain table styling) an SVG image is preferred.

Inline LaTeX is usable in Pandoc's Markdown but is completely ignored when creating an EPub document, thus such version of the book will miss content.

### Special Commands

To simplify the writing of the book as much as possible, I have included some keywords that will pre-processed by Pandoc's filter system:

- `{{paidprod}}`: shows a red `[P]`, used to show a paid product;
- `{{freeprod}}`: shows a green `[F]`, used to show a free product;
- `{{donprod}}`: shows an orange `[D]`, used to show partially free products or products that accept donations;
- `{{placeholder}}`: A simple placeholder, prints the "This section is still a work in progress" message;
- `{{pagebreak}}`: A forced pagebreak, used between chapters.

Also there are some special classes you can use to make boxes:

- `tip`: This class allows to create a "tip box", showing a highlighted box to write your tips in;
- `pitfall`: This class allows to create a "pitfall box", showing a highlighted box to write any traps and pitfalls of a proposed solution.
- `trivia`: This class allows to create a "trivia box", giving some information and trivia about something, which may give better understanding.

These boxes can be created using Pandoc's "fenced div" syntax, like follows:

```
::::: trivia :::::
This is a trivia box
:::::
```

### Dynamic Code Blocks

This book is structured in a way that allows to create many "editions" by specifying a programming language, this means that code is not written directly inside the markdown documents, but instead it is separated in `.txt` files in the `dynamic_listings` folder.

To create a new dynamic code listing just write the following:

~~~
```{src="path/without/language/name" caption="The caption"}
```
~~~

### Unnumbered sections

You may notice that (usually in the appendices), there are titles looking like the following:

```
This is a title {-}
-------------------
```

That `{-}` defines an "unnumbered section", which is mostly an aesthetic choice that I used to make some appendices (like the Glossary) clearer.

### Problems? Questions?

The most common questions will be usually answered in the `FAQ_GUIDES.md` document, for the rest, feel free to open an issue on either GitHub or GitLab.
