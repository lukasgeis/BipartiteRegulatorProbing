#!/bin/bash
#SBATCH --job-name=bprdata
#SBATCH --partition=general1
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=1
#SBATCH --mem-per-cpu=1200
#SBATCH --time=100:00:00
#SBATCH --no-requeue
#SBATCH --mail-type=FAIL

python3 ../scripts/createData.py --number 30 --na_min 10 --na_max 99 --nb_min 10 --nb_max 99 --vs_min 1 --vs_max 1 --output /scratch/memhierarchy/geis/bpr/data/1/0
python3 ../scripts/createData.py --number 30 --na_min 10 --na_max 99 --nb_min 10 --nb_max 99 --vs_min 2 --vs_max 9 --output /scratch/memhierarchy/geis/bpr/data/1/1
python3 ../scripts/createData.py --number 30 --na_min 10 --na_max 99 --nb_min 10 --nb_max 99 --vs_min 10 --vs_max 99 --output /scratch/memhierarchy/geis/bpr/data/1/2

python3 ../scripts/createData.py --number 30 --na_min 100 --na_max 999 --nb_min 100 --nb_max 999 --vs_min 1 --vs_max 1 --output /scratch/memhierarchy/geis/bpr/data/2/0
python3 ../scripts/createData.py --number 30 --na_min 100 --na_max 999 --nb_min 100 --nb_max 999 --vs_min 2 --vs_max 9 --output /scratch/memhierarchy/geis/bpr/data/2/1
python3 ../scripts/createData.py --number 30 --na_min 100 --na_max 999 --nb_min 100 --nb_max 999 --vs_min 10 --vs_max 99 --output /scratch/memhierarchy/geis/bpr/data/2/2

python3 ../scripts/createData.py --number 30 --na_min 1000 --na_max 9999 --nb_min 1000 --nb_max 9999 --vs_min 1 --vs_max 1 --output /scratch/memhierarchy/geis/bpr/data/3/0
python3 ../scripts/createData.py --number 30 --na_min 1000 --na_max 9999 --nb_min 1000 --nb_max 9999 --vs_min 2 --vs_max 9 --output /scratch/memhierarchy/geis/bpr/data/3/1
python3 ../scripts/createData.py --number 30 --na_min 1000 --na_max 9999 --nb_min 1000 --nb_max 9999 --vs_min 10 --vs_max 99 --output /scratch/memhierarchy/geis/bpr/data/3/2
