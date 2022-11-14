Want to contribute? Thank you!
==============================

How to contribute
-----------------

If you want to contribute to this project, you can follow this procedure.

1) Fork this repository to your own account
2) Check out the *develop* branch with `git checkout develop` or your tool of choice
3) Create a new branch with a descriptive name, for instance: `git checkout -b fixes/procedural`
4) Write your contribution
5) Create a Pull Request on the main repository.

Contribution Guidelines
-----------------------

There are some guidelines your contributions should meet to be added to the book.

- Algorithms **must** be written in pseudo-code first, in a platform agnostic way: the pseudocode version is the one that all the language-specific versions are based on. If you want to put other type of code, like C++ or Python, you can, in the appropriate "listings" folders but there must be a pseudo-code version.
- The most important algorithms (like search algorithms or graph navigation ones) will greatly benefit from a big-O estimate of their worst case (although this won't be mandatory), other estimates in addition to this one are welcome (slices of code don't necessarily need estimates, but you can add them if you think they would help). This will help the reader distinguish between better and worse-performing algorithms.
- You should avoid using overly technical terms, if possible. If that is not possible, then you should define such term in the "Glossary" appendix of the book. Check the wiki for more information (it's not hard, we just need to work around some markdown subtleties).
- If you want to add a `{{placeholder}}`, remember to follow that with a `<!-- TODO: ..... -->` so that it's easy to understand what's missing.

These are not strict requirements, they won't make your pull request get rejected, but you may be asked to tweak some things.

Tools needed
------------

Compared to building the book, you may need some more tools to be able to make read all the file formats used in this book.

- **Any text editor** that supports Markdown: for editing the chapters and code listings
- **Inkscape** (or an equivalent SVG/PDF editor): for editing svg images
- **Pandoc**: for building the book
- **TeXLive** or equivalent: for building the PDF Version
- **GNU Make** or equivalent: for using the MakeFile Scripts
- **GladTex** (v3 or higher): for converting math formulas into SVG for EPub
- **Librsvg**: for converting SVGs into a format suitable for TeXLive
- **PlantUML**: for UML diagrams in the `*.puml` format
- **Gimp**: for editing images in the `*.xcf` format
- **GraphViz** (required for PlantUml): for editing diagrams in the `*.dot` format
- **Git**: for version control
- **Any PDF Reader**: just to make sure that the PDF looks good
- **Any EPub Reader**: just to make sure that the EPub looks equally good

Knowledge (kinda) needed
------------------------

If you want to contribute more than just to the content of this project, you may want (but not necessarily **need**) to know:

- **HTML and CSS**: for the EPub template
- **LaTeX**: for the PDF template
- **Lua**: for Pandoc Filters
- **Python**: for some scripts
- **MakeFile Syntax**
- **YAML**: for the ebook metadata (is very self-explanatory, don't worry about this)

Directory Structure
-------------------

This is the directory structure of the project, along with some descriptions that may help you understanding them.

- **chapters**: this is where the main body of the book is written; all the theory and formulas are here, written in Pandoc Markdown
- **dynamic_listings**: this is where you will find the code listings, they will be replaced when the book gets "compiled", thus making the "editions"
- **filters**: this directory contains Pandoc's Lua filters, used to fill in the code listings, epigraphs, placeholders, page breaks, symbols and highlighted sections (boxes)
- **images**: this folder contains the images that will be inserted in the book
- **scripts**: this folder contains some scripts that may be useful to maintain the book
- **template**: this folder contains the LaTeX and HTML templates (for PDF and EPub output)

You can also find other folders, such as:

- **raw_resources**: this folder contains some raw resources used to build the book, usually the code used to generate UML and dot diagrams, or XCF files used to create some images.
- **gladtex_imgs**: this is a temporary folder used by the GladTeX tool to convert math formulas into SVG images (thus improving compatibility with EPub readers).

Adding a new programming language
---------------------------------

This book is built in a way that allows everyone to have the book in their own favourite programming language, as long as the listings for such language are available. LaTeX will take care of the highlighting.

If you want to add a new programming language, navigate to the `listings` folder, copy the `pseudocode` folder (or any other code folder you want) and name it with your language's name. The languages that feature syntax highlighting are the ones supported by [skylighting](https://github.com/jgm/skylighting) and LaTeX's "listings" package.

The only thing left to do is translating all the pseudo-code listings into your favourite programming language.

After that you can try and make a copy of the book, by editing the `Makefile`. Remember to add the new language in the `README.md` file.

You may also want to refer to the "Dynamic Code Blocks" section of this document.

Translating the book
--------------------

If you want to take over the task of translating the book into another language, here's something you should be mindful of:

- Translations are considered "derivative work" and should be distributed under the same license of this book
- Translating the **chapters** content will translate the theory part of the book
- Translating the **dynamic listings** directory is necessary, since comments are in English
- Some images may need translation
- This book is an eternal "work in progress", so you'll have to expect changes.

Currently there is no "centralized" way to handle translations, so please open an issue and we can work out a solution.

Programming Language Addons / Extensions
----------------------------------------

You can add new listings that include your favourite framework/library by creating a new folder in the language of choice and using the `lang_extension` metavariable (just add `-M lang_extension="your_folder_name"`) to the commands in the MakeFile.

An example is worth a thousand words: let's pretend we want to add support for Pygame, a library for game development in python.

1. Go to the `dynamic_listings/python/` directory and create a new folder called "pygame"
2. Copy the contents of the `default` subdirectory and change them according to the library you're writing for (in this case, Pygame)
3. Go back to the root of the project and edit the `Makefile` file, by copying the default python command and adding the lang extension metavariable. It should look something like this:

```
[... other code here ...]

python_pygame:
	$(PANDOC_STANDALONE) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) $(PDF_TEMPLATE) -M proglang=python -M lang_extension=pygame -o Python_Pygame_Edition.pdf

[... more code here ...]
```

4. Now you can build your "extended edition" by using the make commands you just created (in this case `make python_pygame`).

### What if my programming language is not supported?

No worries, you can still add your own. Check the [FAQ Document](FAQ_GUIDES.md).

Pictures
--------

The pictures used should be simple to understand and not covered by copyright (screenshots of real games should be avoided if possible, as copyright and the "fair use" doctrine are quite complicated), where possible images should be saved in the following formats:

- **Diagrams:** Vector SVG format (possibly optimized) - Some people may find useful to zoom into an image to better understand it and they usually have a low size;
- **Picture Examples:** PNG Format, they have a good quality/compression tradeoff.

The most important pictures *should* have a caption to describe them, but not necessarily, that's up to you.

Images that are subject to change and corrections (like UML diagrams) should include a raw format inside the `raw_resources/images/` folder.

Markdown Vs. LaTeX
------------------

The book should be written as much as possible in [Pandoc Markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), because everyone should be able to contribute to this book easily.

If the situation calls for it (for instance you *absolutely need* a certain table styling) an SVG image is preferred.

Inline LaTeX is usable in Pandoc's Markdown but is completely ignored when creating an EPub document, this means that such version of the book will miss content.

Special Commands
----------------

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
- `note`: This class allows to create a "note", which doesn't fit any of the previous cases. This is used when the content still needs to be highlighted in some way.

These boxes can be created using Pandoc's "fenced div" syntax, like follows:

```
::::: trivia :::::
This is a trivia box
::::::::::::::::::
```

Dynamic Code Blocks
-------------------

This book is structured in a way that allows to create many "editions" by specifying a programming language, this means that code is not written directly inside the markdown documents, but instead it is separated in `.txt` files in the `dynamic_listings` folder. This allows for easily "switching" code listings without having many copies of the repository.

To create a new dynamic code listing just write the following:

~~~
```{src="path/without/language/name" caption="The caption"}
```
~~~

Unnumbered sections
-------------------

You may notice that (usually in the appendices), there are titles looking like the following:

```
This is a title {.unnumbered .unlisted}
-------------------
```

That `{.unnumbered .unlisted}` defines an "unnumbered and unlisted section", which is mostly an aesthetic choice that I used to make some appendices (like the Glossary) appear in a cleaner manner in the Table of Contents.

Miscellaneous
-------------

If you want to add tools or engines, try to be detailed while keeping it short. Engines and tools are written in alphabetical order.

Problems? Questions?
--------------------

The most common questions will be usually answered in the `FAQ_GUIDES.md` document, for the rest, feel free to open an issue on either GitHub or GitLab.
