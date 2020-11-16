all: pseudocode pseudocode_color python cpp

pseudocode:
	pandoc --listings -N `ls chapters/*.md | sort -V` metadata.yaml -M version=`git describe --tags` -M proglang="" --template template/template.tex -s -o Book_pseudocode.pdf

pseudocode_color:
	pandoc --listings -N `ls chapters/*.md | sort -V` metadata.yaml -M version=`git describe --tags` -M proglang=pseudocode --template template/template.tex -s -o Book_pseudocode_colored.pdf

python:
	pandoc --listings -N `ls chapters/*.md | sort -V` metadata.yaml -M version=`git describe --tags` -M proglang=python --template template/template.tex -s -o Book_python.pdf

cpp:
	pandoc --listings -N `ls chapters/*.md | sort -V` metadata.yaml -M version=`git describe --tags` -M proglang=C++ --template template/template.tex -s -o Book_cpp.pdf

clean:
	rm -f *.pdf *.aux *.toc *.lol *.lot *.log *.out
	echo "Pulizia Completa"
