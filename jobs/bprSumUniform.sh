#!/bin/bash
#SBATCH --job-name=bprSumUniform
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=64
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/bpr/paper/uniform/sum"

for NUM in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
do
    cargo run --release -- --log $OUTPUTDIR --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 100 --instances 10 --goal SUM
done