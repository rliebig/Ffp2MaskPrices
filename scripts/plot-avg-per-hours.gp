set terminal png
set output "hourly-barchart.png"
set boxwidth 0.5
set style fill solid
set xtics 0,1,23
plot "hourly.txt" with boxes