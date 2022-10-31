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
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce10 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce11 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce & 
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce12 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/2/0/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce20 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce21 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce22 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/3/0/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce30 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce31 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/namp/bruteforce32 --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 10 --algorithm NAMP --bruteforce &
done