#!/bin/bash

for i in {2..20}
do
    L=$((10*$i))
    cargo run --release -- -t 20 -l $L -n $((300*40*$L)) -r 0.2
done