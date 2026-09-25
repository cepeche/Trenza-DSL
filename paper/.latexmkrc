# latexmk configuration for Trenza ONWARD! paper
# Usage: latexmk -pdf main.tex
#        latexmk -pdf -pvc main.tex   (continuous preview)
#        latexmk -C                   (clean all generated files)

$pdf_mode = 1;          # generate PDF via pdflatex
$pdflatex = 'pdflatex -interaction=nonstopmode -synctex=1 %O %S';

# Extra file extensions to clean with latexmk -C
$clean_ext = 'synctex.gz synctex.gz(busy) run.xml tex.bak bbl bcf fdb_latexmk run tdo %R-blx.bib';
