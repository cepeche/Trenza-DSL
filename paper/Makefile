# Trenza paper — Makefile
# Requires: latexmk, pdflatex (TeX Live or MiKTeX)

.PHONY: all clean watch

all:
	latexmk -pdf main.tex

watch:
	latexmk -pdf -pvc main.tex

clean:
	latexmk -C
	rm -f *.aux *.log *.bbl *.bcf *.blg *.run.xml *.synctex.gz
