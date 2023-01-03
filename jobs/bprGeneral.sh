#!/bin/bash
#SBATCH --job-name=bprGeneral
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=60
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/bpr/experiments/general"

for NUM in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
do
    target/release/bpr --log "${OUTPUTDIR}/max/OUT_${NUM}" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 500 --instances 20 --goal MAX --algorithm ALL --parameters 4 &
    target/release/bpr --log "${OUTPUTDIR}/sum/OUT_${NUM}" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 500 --instances 20 --goal SUM --algorithm ALL --parameters 4 &
    target/release/bpr --log "${OUTPUTDIR}/cov/OUT_${NUM}" --na $((16 * $NUM)) --nb $((16 * $NUM)) --vs $((16 * $NUM)) --iterations 500 --instances 20 --goal COV --algorithm ALL --parameters 4 &
done
