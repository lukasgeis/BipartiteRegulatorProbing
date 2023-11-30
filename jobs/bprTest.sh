#!/bin/bash
#SBATCH --job-name=bprTest
#SBATCH --partition=test
#SBATCH --nodes=1
#SBATCH --ntasks=10
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=00:01:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/bpr/paper/test"

for NUM in 1 2 3 4 5
do
    cargo run --release -- --log $OUTPUTDIR --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 10 --instances 10 --goal MAX 
    cargo run --release -- --log $OUTPUTDIR --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 10 --instances 10 --goal SUM 
    cargo run --release -- --log $OUTPUTDIR --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 10 --instances 10 --goal COV 
done