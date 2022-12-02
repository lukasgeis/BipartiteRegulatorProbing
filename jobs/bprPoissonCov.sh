#!/bin/bash
#SBATCH --job-name=bprPoissonCov
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=20
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/bpr/experiments/poisson/cov1"

for NUM in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
do
    target/release/bpr --log "${OUTPUTDIR}/OUT_${NUM}" --na $((16 * $NUM)) --nb $((400 * $NUM)) --vs 10 --iterations 200 --instances 20 --goal COV --algorithm FAST --parameters 4 --poisson &
done

wait