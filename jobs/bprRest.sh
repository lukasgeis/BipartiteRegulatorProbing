#!/bin/bash

OUTPUTDIR="data"

ITERATIONS=100
INSTANCES=10 

COMMON="--iterations $ITERATIONS --instances $INSTANCES"

cargo build --release

BEG=10 
END=20

# Cov Network
for NUM in $(seq $BEG $END) 
do 
    echo "Run COV - Network - $NUM"
    ./target/release/bpr --log "$OUTPUTDIR/cov/network" --na $((16 * $NUM)) --nb $((400 * $NUM)) --vs 10 $COMMON --goal COV --poisson
done
