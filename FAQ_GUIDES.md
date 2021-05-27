F.A.Q. and Guides
=================

Are there any differences with the standard Pandoc Markdown and the language used in this book?
-----------------------------------------------------------------------------------------------

There are a few, but mostly the language is standard Pandoc markdown, with a few added features.

### Code fences

~~~
```{src="path/without/language/name" caption="The caption"}
```
~~~

This is used to insert a code fence, the make script, the lua filters and pandoc will take care of swapping out the code listings depending on the edition you are building.

### Keywords

`{{paidprod}}` is used to identify a "Paid Product" in the resources list.

`{{freeprod}}` is used to identify a "Free Product" in the resources list.

`{{donprod}}` is used to identify a "Product that accepts donations" or a "Partially free product" in the resources list.

`{{placeholder}}` is use to print a "placeholder" in a section of the book.

I can't export my diagrams to Vector SVG, what do I do?
--------------------------------------------------------

If you can't export your diagrams in SVG, try exporting them in the `*.eps` format and then use a vector graphics tool like Inkscape to save them in "Optimized SVG". You can leave the default settings.

Adding a new programming language
---------------------------------

Due to the duplicitous nature of this book (PDF and EPub), now you need to refer to the [Skylighting Documentation](https://github.com/jgm/skylighting#adding-new-syntaxes) to add new languages.

Don't worry though! Skylighting is based on tokenizers derived from the well maintained KDE project and they are made in XML, so it should be pretty easy to make a new one and ask the Skylighting developer to add it to the base package.
