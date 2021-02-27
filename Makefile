PANDOC=pandoc
PANDOC_STANDALONE=pandoc -s
CHAPTERS_CMD=`find chapters/*.md | sort -V` metadata.yaml
PANDOC_DEFAULT_ARGS=--lua-filter ./filters/filter_helper.lua --listings -N --pdf-engine=xelatex
PDF_TEMPLATE=--template template/template.tex
EPUB_TEMPLATE=--css template/epub.css
VERSION=-M version=`git describe --tags`
GLADTEX_PKG=gladtex -d "gladtex_imgs" -P -p "\usepackage{cancel}\usepackage{gensymb}" -

all: pseudocode pseudocode_color python cpp

pseudocode:
	$(PANDOC_STANDALONE) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) $(PDF_TEMPLATE) -M proglang="" -o Book_pseudocode.pdf

pseudocode_color:
	$(PANDOC_STANDALONE) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) $(PDF_TEMPLATE) -M proglang=pseudocode -o Book_pseudocode_colored.pdf

python:
	$(PANDOC_STANDALONE) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) $(PDF_TEMPLATE) -M proglang=python -o Book_python.pdf

cpp:
	$(PANDOC_STANDALONE) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) $(PDF_TEMPLATE) -M proglang=C++ -o Book_cpp.pdf

latex:
	$(PANDOC_STANDALONE) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) $(PDF_TEMPLATE) -M proglang="" -o Book_LaTeX.latex

epub:
	$(PANDOC) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) -M proglang="" -t json | $(GLADTEX_PKG) | $(PANDOC_STANDALONE) -f json $(EPUB_TEMPLATE) -o Book_Epub.epub

.PHONY: clean
clean:
	rm -f *.pdf *.aux *.toc *.lol *.lot *.log *.out *.latex outsourced_descriptions.html gladtex_imgs/*
	echo "Pulizia Completa"
