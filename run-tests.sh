#!/bin/bash

for i in {2..20}
do
    L=$((10*$i))
    cargo run --release -- -m -t 20 -l $L -n 1000 -r 0.3 -b 150
done