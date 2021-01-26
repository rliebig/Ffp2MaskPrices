#!/bin/bash
python Ffp2MaskPrices/scripts/gnuplot-preprocess.py
gnuplot Ffp2MaskPrices/scripts/plot-avg.gp
python3 -m http.server