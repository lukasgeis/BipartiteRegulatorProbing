#!/bin/bash
#SBATCH --job-name=bprCov
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=24
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

OUTPUTDIR="/scratch/memhierarchy/geis/logs/bpr/cov"

for NUM in 3 4 5 6 7 8 9 10
do
    target/release/bpr --log "${OUTPUTDIR}/COV_${NUM}_1" --na $((2**$NUM)) --nb $((2**$NUM)) --vs $NUM --iterations 10 --instances 20 --goal COV --algorithm FAST --parameters $((2**$NUM / 2)) &
    target/release/bpr --log "${OUTPUTDIR}/COV_${NUM}_2" --na $((2**$NUM)) --nb $((2**$NUM)) --vs $((2 * $NUM)) --iterations 10 --instances 20 --goal COV --algorithm FAST --parameters $((2**$NUM / 2)) &
    target/release/bpr --log "${OUTPUTDIR}/COV_${NUM}_3" --na $((2**$NUM)) --nb $((2**$NUM)) --vs $((4 * $NUM)) --iterations 10 --instances 20 --goal COV --algorithm FAST --parameters $((2**$NUM / 2)) &
done
