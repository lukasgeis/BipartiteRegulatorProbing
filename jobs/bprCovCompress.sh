#!/bin/bash
#SBATCH --job-name=bprCovCompress
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=2000
#SBATCH --time=05:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

python3 scripts/compressCovData.py --input /scratch/memhierarchy/geis/bpr/logs/cov --output /scratch/memhierarchy/geis/bpr/logs/COV_COMPRESSED