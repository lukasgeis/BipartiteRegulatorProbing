#!/bin/bash
#SBATCH --job-name=bprPOLY
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
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/10/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/11/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/1/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/12/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/2/0/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/20/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/21/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/2/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/22/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait


for file in /scratch/memhierarchy/geis/bpr/data/3/0/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/30/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/31/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done

wait

for file in /scratch/memhierarchy/geis/bpr/data/3/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/poly/32/out_$file --iterations 10 --algorithm POLY --fraction 4 &
done