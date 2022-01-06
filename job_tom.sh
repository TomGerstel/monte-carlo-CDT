#!/bin/bash
#SBATCH --partition=hefstud
#SBATCH --output=./cluster_output/std_%A_%a.txt
#SBATCH --mem=100M
#SBATCH --time=3:00:00
#SBATCH --array=0-9%3
cd ~/Documents/monte-carlo-CDT
Ts=($(LANG=en_US seq 10 10 100))
T=${Ts[$SLURM_ARRAY_TASK_ID]}
tcors=($(LANG=en_US seq 40 40 400))
tcor=${tcors[$SLURM_ARRAY_TASK_ID]}
./target/release/monte-carlo-cdt -m -t ${T} -l 200 -n 100 -r 0.4 -b 200 -p ${tcor} -o "./data"
