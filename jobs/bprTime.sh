#!/bin/bash
#SBATCH --job-name=bprFast
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=90
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/bpr/experiments/general"

for NUM in 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50
do
    target/release/bpr --log "${OUTPUTDIR}/max/OUT_${NUM}" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 500 --instances 10 --goal MAX --algorithm ALL --parameters 4 &
    target/release/bpr --log "${OUTPUTDIR}/sum/OUT_${NUM}" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 500 --instances 10 --goal SUM --algorithm ALL --parameters 4 &
    target/release/bpr --log "${OUTPUTDIR}/cov/OUT_${NUM}" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 500 --instances 10 --goal COV --algorithm ALL --parameters 4 &
done

wait