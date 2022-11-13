#!/bin/bash
#SBATCH --job-name=bprPoly
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=16
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=1200
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for NUM in 3 4 5 6 7 8 9 10
do
    target/release/bpr --log /scratch/memhierarchy/geis/bpr/logs/poly/MAX_$NUM --na $((2**$NUM)) --nb $((2**$NUM)) --vs $NUM --iterations 5 --instances 10 --goal MAX --algorithm POLY --parameters $((2**$NUM - 2)) &
    target/release/bpr --log /scratch/memhierarchy/geis/bpr/logs/poly/SUM_$NUM --na $((2**$NUM)) --nb $((2**$NUM)) --vs $NUM --iterations 5 --instances 10 --goal SUM --algorithm POLY --parameters $((2**$NUM - 2)) &
done
