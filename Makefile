pdf:
	pandoc --listings -N `ls chapters/*.md | sort -V` metadata.yaml --template template/template.tex -s -o Book.pdf

clean:
	rm -f *.pdf
	echo "Pulizia Completa"

