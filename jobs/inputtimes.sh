#!/bin/bash
#SBATCH --job-name=bprinput
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=30
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=100000
#SBATCH --time=60:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for file in /scratch/memhierarchy/geis/bpr/data/0/0/*
do 
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 0-0"

for file in /scratch/memhierarchy/geis/bpr/data/0/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT & 
done

wait
echo "Finished 0-1"

for file in /scratch/memhierarchy/geis/bpr/data/0/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 0-2"

for file in /scratch/memhierarchy/geis/bpr/data/1/0/*
do 
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 1-0"

for file in /scratch/memhierarchy/geis/bpr/data/1/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT & 
done

wait
echo "Finished 1-1"

for file in /scratch/memhierarchy/geis/bpr/data/1/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 1-2"


for file in /scratch/memhierarchy/geis/bpr/data/2/0/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 2-0"

for file in /scratch/memhierarchy/geis/bpr/data/2/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 2-1"

for file in /scratch/memhierarchy/geis/bpr/data/2/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 2-2"

for file in /scratch/memhierarchy/geis/bpr/data/3/0/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 3-0"

for file in /scratch/memhierarchy/geis/bpr/data/3/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 3-1"

for file in /scratch/memhierarchy/geis/bpr/data/3/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --input-time /scratch/memhierarchy/geis/bpr/logs/inputtimes --iterations 0 --algorithm OPT &
done

wait
echo "Finished 3-2"