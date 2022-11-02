#!/bin/bash
#SBATCH --job-name=bprall
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=9
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=10000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for file in /scratch/memhierarchy/geis/bpr/data/1/0/Random_na1*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/all/OUT_$BASENAME --iterations 10 --algorithm ALL --fraction 4 &
done

for file in /scratch/memhierarchy/geis/bpr/data/1/0/Random_na2*
do
    BASENAME="$(basename -- $file)"
    target/release/bpr --input $file --log /scratch/memhierarchy/geis/bpr/logs/all/OUT_$BASENAME --iterations 10 --algorithm ALL --fraction 4 &
done
