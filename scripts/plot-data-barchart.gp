# arguments are possible by using -e "filename=data12.txt"
set terminal png
set output "barchart.png"
set boxwidth 0.5
set style fill solid
plot filename with boxes