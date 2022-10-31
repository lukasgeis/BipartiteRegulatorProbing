#!/bin/bash
#SBATCH --job-name=bprPOLY
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=1200
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for file in /scratch/memhierarchy/geis/bpr/data/1/0/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison10 --iterations 10 --algorithm POLY --fraction 4
done

for file in /scratch/memhierarchy/geis/bpr/data/1/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison11 --iterations 10 --algorithm POLY --fraction 4
done

for file in /scratch/memhierarchy/geis/bpr/data/1/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison12 --iterations 10 --algorithm POLY --fraction 4
done


for file in /scratch/memhierarchy/geis/bpr/data/2/0/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison20 --iterations 10 --algorithm POLY --fraction 4
done

for file in /scratch/memhierarchy/geis/bpr/data/2/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison21 --iterations 10 --algorithm POLY --fraction 4
done

for file in /scratch/memhierarchy/geis/bpr/data/2/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison22 --iterations 10 --algorithm POLY --fraction 4
done


for file in /scratch/memhierarchy/geis/bpr/data/3/0/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison30 --iterations 10 --algorithm POLY --fraction 4
done

for file in /scratch/memhierarchy/geis/bpr/data/3/1/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison31 --iterations 10 --algorithm POLY --fraction 4
done

for file in /scratch/memhierarchy/geis/bpr/data/3/2/*
do
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/POLY/comparison32 --iterations 10 --algorithm POLY --fraction 4
done