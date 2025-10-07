#!/bin/bash

OUTPUTDIR="data"

ITERATIONS=100
INSTANCES=10 

COMMON="--iterations $ITERATIONS --instances $INSTANCES"

cargo build --release

END=20

# Max Uniform
for NUM in $(seq 1 $END) 
do 
    # MAX 
    echo "Run MAX - Uniform - $NUM"
    ./target/release/bpr --log "$OUTPUTDIR/max/uniform" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) $COMMON --goal MAX 
done

# Sum Uniform
for NUM in $(seq 1 $END) 
do 
    # SUM
    echo "Run SUM - Uniform - $NUM"
    ./target/release/bpr --log "$OUTPUTDIR/sum/uniform" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) $COMMON --goal SUM
done

# Cov Uniform
for NUM in $(seq 1 $END) 
do 
    # COV 
    echo "Run COV - Uniform - $NUM"
    ./target/release/bpr --log "$OUTPUTDIR/cov/uniform" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) $COMMON --goal COV --ipopt
done

# Max Network
for NUM in $(seq 1 $END) 
do 
    echo "Run MAX - Network - $NUM"
    ./target/release/bpr --log "$OUTPUTDIR/max/network" --na $((16 * $NUM)) --nb $((400 * $NUM)) --vs 10 $COMMON --goal MAX --poisson
done

# Sum Network 
for NUM in $(seq 1 $END) 
do 
    echo "Run SUM - Network - $NUM"
    ./target/release/bpr --log "$OUTPUTDIR/sum/network" --na $((16 * $NUM)) --nb $((400 * $NUM)) --vs 10 $COMMON --goal SUM --poisson
done

# Cov Network
for NUM in $(seq 1 $END) 
do 
    echo "Run COV - Network - $NUM"
    ./target/release/bpr --log "$OUTPUTDIR/cov/network" --na $((16 * $NUM)) --nb $((400 * $NUM)) --vs 10 $COMMON --goal COV --poisson --ipopt
done
