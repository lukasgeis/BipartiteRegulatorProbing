#!/bin/bash
#SBATCH --job-name=bprAll
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=6
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

for NUM in 3 4 5 
do
    target/release/bpr --log /scratch/memhierarchy/geis/bpr/logs/all/MAX_$NUM --na $((2**$NUM)) --nb $((2**$NUM)) --vs $NUM --iterations 10 --instances 20 --goal MAX --algorithm ALL --parameters $((2**$NUM - 2)) &
    target/release/bpr --log /scratch/memhierarchy/geis/bpr/logs/all/SUM_$NUM --na $((2**$NUM)) --nb $((2**$NUM)) --vs $NUM --iterations 10 --instances 20 --goal SUM --algorithm ALL --parameters $((2**$NUM - 2)) &
done

