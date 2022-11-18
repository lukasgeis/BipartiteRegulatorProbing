#!/bin/bash
#SBATCH --job-name=bprPoissonCov
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=20
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=25:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/logs/bpr/cov"

for NUM in 1 2 3 4 5 6 7 8 9 10 20 30 40 50 60 70 80 90 100 125
do
    target/release/bpr --log "${OUTPUTDIR}/COV_POISSON_${NUM}" --na $((16 * $NUM)) --nb $((400 * $NUM)) --vs 10 --iterations 100 --instances 50 --goal COV --algorithm FAST --parameters 4 &
done
