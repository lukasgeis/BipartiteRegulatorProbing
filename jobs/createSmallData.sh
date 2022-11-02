#!/bin/bash
#SBATCH --job-name=bprdatasmall
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=3
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=1200
#SBATCH --time=00:01:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

python3 scripts/createData.py --number 30 --na_min 3 --na_max 10 --nb_min 2 --nb_max 10 --vs_min 1 --vs_max 1 --output /scratch/memhierarchy/geis/bpr/data/0/0 &
python3 scripts/createData.py --number 30 --na_min 3 --na_max 10 --nb_min 2 --nb_max 10 --vs_min 2 --vs_max 2 --output /scratch/memhierarchy/geis/bpr/data/0/1 &
python3 scripts/createData.py --number 30 --na_min 3 --na_max 10 --nb_min 2 --nb_max 10 --vs_min 3 --vs_max 3 --output /scratch/memhierarchy/geis/bpr/data/0/2 &

