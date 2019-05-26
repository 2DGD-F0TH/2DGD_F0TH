pdf:
	pandoc --listings -N chapters/*.md metadata.yaml --template template/template.tex -s -o Book.pdf

clean:
	rm -f *.pdf
	echo "Pulizia Completa"

