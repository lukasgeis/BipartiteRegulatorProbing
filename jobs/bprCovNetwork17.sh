#!/bin/bash
#SBATCH --job-name=bprCovNetwork17
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=64
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=150:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/bpr/paper/network/cov17"

for NUM in 17
do
    cargo run --release -- --log $OUTPUTDIR --na $((16 * $NUM)) --nb $((400 * $NUM)) --vs 10 --iterations 100 --instances 10 --goal COV --poisson
done