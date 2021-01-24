set terminal png
set output "myfile.png"
set style line 1 lc rgb "#0069ad" lt 1 lw 2 pt 7 pi -1 ps 1.5
set pointintervalbox 3
plot "newavg.txt" with linespoints ls 1