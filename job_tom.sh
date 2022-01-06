#!/bin/bash
#SBATCH --partition=hefstud
#SBATCH --output=./cluster_output/std_%A_%a.txt
#SBATCH --mem=100M
#SBATCH --time=3:00:00
#SBATCH --array=1-10%3
cd ~/Documents/monte-carlo-CDT
T=$((10*$SLURM_ARRAY_TASK_ID))
tcor=$((4*$T))
./target/release/monte-carlo-cdt -m -t $T -l 200 -n 100 -r 0.4 -b 200 -p $tcor -o "./data"