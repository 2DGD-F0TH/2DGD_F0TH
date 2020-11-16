PANDOC=pandoc
CHAPTERS_CMD=`ls chapters/*.md | sort -V` metadata.yaml
PANDOC_DEFAULT_ARGS=--listings -N --template template/template.tex -s
VERSION=-M version=`git describe --tags`

all: pseudocode pseudocode_color python cpp

pseudocode:
	$(PANDOC) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) -M proglang="" -o Book_pseudocode.pdf

pseudocode_color:
	$(PANDOC) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) -M proglang=pseudocode -o Book_pseudocode_colored.pdf

python:
	$(PANDOC) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) -M proglang=python -o Book_python.pdf

cpp:
	$(PANDOC) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) -M proglang=C++ -o Book_cpp.pdf

latex:
	$(PANDOC) $(PANDOC_DEFAULT_ARGS) $(CHAPTERS_CMD) $(VERSION) -M proglang="" -o Book_LaTeX.latex

.PHONY: clean
clean:
	rm -f *.pdf *.aux *.toc *.lol *.lot *.log *.out *.latex
	echo "Pulizia Completa"
