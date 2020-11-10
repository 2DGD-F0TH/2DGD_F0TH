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

- Algorithms **must** be written in pseudo-code first, in a platform agnostic way: the pseudocode version is the one that is published by the CI/CD system. If you want to put other type of code, like C++ or Python, you can, in the appropriate "listings" folders but there must be a pseudo-code version.
- The most important algorithms (like search algorithms or graph navigation ones) **must** have at least a big-O estimate of their worst case, other estimates in addition to this one are welcome (slices of code don't necessarily need estimates, but you can add them if you think they would help). This will help the reader distinguish between better and worse-performing algorithms.
- You should avoid using overly technical terms, if possible. If that is not possible, then you should define such term in the "Glossary" appendix of the book and add a ~[g]~ at the end of the term (so that the reader knows such term is in the glossary).

These are not strict requirements, they won't make your pull request get rejected, but you may be asked to tweak some things.

### Adding a new programming language

This book is built in a way that allows everyone to have the book in their own favourite programming language, as long as the listings for such language are available. LaTeX will take care of the highlighting.

If you want to add a new programming language, navigate to the `listings` folder, copy the `pseudocode` folder and name it with your language's name - [check this link](https://en.wikibooks.org/wiki/LaTeX/Source_Code_Listings#Supported_languages) for the languages supported by the plugin used.

The only thing left to do is translating all the pseudo-code listings into your favourite programming language.

After that you can try and make a copy of the book, by editing the `Makefile`. Remember to add the new language in the `README.md` file.

#### What if my programming language is not in the list?

No worries, you can still add your own. Check the [FAQ Document](FAQ_GUIDES.md).

### Pictures

The pictures used should be simple to understand and not covered by copyright (screenshots of real games should be avoided if possible, as copyright and the "fair use" doctrine are quite complicated), where possible images should be saved in the following formats:

- **Diagrams:** Vector PDF format - Some people may find useful to zoom into an image to better understand it and they usually have a low size;
- **Picture Examples:** PNG Format, they have a good quality/compression tradeoff.

The most important pictures *should* have a caption to describe them, but not necessarily, that's up to you.

Images that are subject to change and corrections (like UML diagrams) should include a raw format inside the `raw_resources/images/` folder.

### Markdown Vs. LaTeX

The book should be written as much as possible in [Pandoc Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), because everyone should be able to contribute to this book easily.

If the situation calls for it (for instance you *absolutely need* a certain table styling) you can use pure LaTeX inline, inside the markdown document.

### Special Environments and commands

To simplify the writing of the book as much as possible, I have included some commands and LaTeX environments:

- `\code{path}{caption}`: Allows to insert a dynamic code listing, that will source dynamically from the `dynamic_listings` folder, according to the language selected;
- `\placeholder`: A simple placeholder, prints the "This section is still a work in progress" message;
- `\begin{tip}...\end{tip}`: This environment allows to create a "tip box", showing a highlighted box to write your tips in;
- `\begin{pitfall}...\end{pitfall}`: This environment allows to create a "pitfall box", showing a highlighted box to write any traps and pitfalls of a proposed solution.
- `\paidprod`: shows a red `[P]`, used to show a paid product;
- `\freeprod`: shows a green `[F]`, used to show a free product;
- `\donprod`: shows an orange `[D]`, used to show partially free products or products that accept donations;

### Problems? Questions?

The most common questions will be usually answered in the `FAQ_GUIDES.md` document, for the rest, feel free to open an issue on either GitHub or GitLab.
