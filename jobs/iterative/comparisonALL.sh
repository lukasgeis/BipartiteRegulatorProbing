#!/bin/bash
#SBATCH --job-name=bprallit
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=30
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=100000
#SBATCH --time=120:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for file in /scratch/memhierarchy/geis/bpr/data/0/0/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/all/00/OUT_$BASENAME --iterations 10 --algorithm ALL --fraction 4 
done

wait
echo "Finished 0-0"

for file in /scratch/memhierarchy/geis/bpr/data/0/1/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/all/01/OUT_$BASENAME --iterations 10 --algorithm ALL --fraction 4
done

wait
echo "Finished 0-1"

for file in /scratch/memhierarchy/geis/bpr/data/0/2/*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/all/02/OUT_$BASENAME --iterations 10 --algorithm ALL --fraction 4
done

wait
echo "Finished 0-2"
