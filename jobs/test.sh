#!/bin/bash
#SBATCH --job-name=test
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=30
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=200
#SBATCH --time=1:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for file in /scratch/memhierarchy/geis/bpr/data/1/0/*
do 
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/10/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/11/OUT_$BASENAME --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce & 
done