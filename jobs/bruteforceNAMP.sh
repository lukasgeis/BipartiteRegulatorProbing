#!/bin/bash
#SBATCH --job-name=bprnampbf
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=30
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=10000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for file in /scratch/memhierarchy/geis/bpr/data/1/0/*
do 
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/10/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/11/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- & 
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/12/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/2/0/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/20/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/21/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/22/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/3/0/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/30/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/31/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/32/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP -- &
done