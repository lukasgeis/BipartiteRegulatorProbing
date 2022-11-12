#!/bin/bash
#SBATCH --job-name=bprFast
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=10
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=1200
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for NUM in 3 4 5 6 7 8 9 10 11 12
do
    target/release/bpr --log /scratch/memhierarchy/geis/bpr/logs/fast/OUT_$NUM --na $((2**$NUM)) --nb $((2**$NUM)) --vs $NUM --iterations 5 --instances 10 --algorithm FAST --parameters $((2**$NUM - 2)) &
done
