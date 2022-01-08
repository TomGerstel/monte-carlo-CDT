#!/bin/bash
#SBATCH --partition=hefstud
#SBATCH --output=./cluster_output/std_%A_%a.txt
#SBATCH --mem=100M
#SBATCH --time=3:00:00
#SBATCH --array=0-3%4
cd ~/Documents/monte-carlo-CDT
Ls=($(LANG=en_US seq 15 5 30))
Ts=($(LANG=en_US seq 300 100 600))
tcors=($(LANG=en_US seq 60 20 120))
L=${Ls[$SLURM_ARRAY_TASK_ID]}
T=${Ts[$SLURM_ARRAY_TASK_ID]}
tcor=${tcors[$SLURM_ARRAY_TASK_ID]}
./target/release/monte-carlo-cdt -m -t $T -l $L -n 100 -r 0.4 -b 200 -p $tcor -o "./data"