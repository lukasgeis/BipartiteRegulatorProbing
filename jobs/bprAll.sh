#!/bin/bash
#SBATCH --job-name=bprAll
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=3
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=1200
#SBATCH --time=120:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for NUM in 3 4 5 
do
    target/release/bpr --log /scratch/memhierarchy/geis/bpr/logs/all/OUT_$NUM --na $((2**$NUM)) --nb $((2**$NUM)) --vs $NUM --iterations 5 --instances 10 --algorithm ALL --parameters $((2**$NUM - 2)) &
done
