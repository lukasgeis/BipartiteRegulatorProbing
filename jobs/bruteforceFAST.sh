#!/bin/bash
#SBATCH --job-name=bprfastbf
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
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/10/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/11/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce & 
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/12/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/2/0/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/20/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/21/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/22/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/3/0/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/30/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/31/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/FAST/32/OUT_$BASENAME --iterations 10 --algorithm FAST --bruteforce &
done