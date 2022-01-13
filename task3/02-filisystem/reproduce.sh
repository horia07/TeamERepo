#!/bin/bash

cd fio
bash benchmark.sh
mv results/plotwrite.png ../RESULTS
mv results/plotread.png ../RESULTS
bash clean.sh	
cd ..

cd phoronix
bash phoronix.sh
sudo mv results/* ../RESULTS
cd ..

cd RESULTS 
mv 001.pdf phoronix.pdf

echo "!!!DON'T FORGET TO COPY THE LINK AND ACCESS THE PHORONIX RESULTS ONLINE FOR BETTER OVERVIEW!!!"

