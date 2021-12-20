#!/bin/bash

# for i in {2..20}
# do
#     L=$((10*$i))
#     cargo run --release -- -m -t 20 -l $L -n 50000 -r 0.3 -b 200 -p 1
# done

# half=5
# for i in {0..9}
# do
#     cargo run --release -- -m -t 20 -l 80 -n 50000 -r "0.$i$half" -b 200 -p 1
# done

for i in {7..10}
do
    T=$((10*$i))
    cargo run --release -- -m -t $T -l 200 -n 50000 -r 0.4 -b 200 -p 1
done