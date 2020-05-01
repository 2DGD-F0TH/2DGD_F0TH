pdf:
	pandoc --listings -N `ls chapters/*.md | sort -V` --metadata-file=metadata.yaml -M version=`git describe --tags` --template template/template.tex -s -o Book.pdf

clean:
	rm -f *.pdf
	echo "Pulizia Completa"

