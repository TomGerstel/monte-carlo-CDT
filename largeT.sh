#!/bin/bash
#SBATCH --partition=hefstud
#SBATCH --output=./cluster_output/std_%A_%a.txt
#SBATCH --mem=100M
#SBATCH --time=15:00:00
#SBATCH --array=0-3%4
cd ~/Documents/monte-carlo-CDT
Ls=($(LANG=en_US seq 35 5 50))
Ts=($(LANG=en_US seq 700 100 1000))
tcors=($(LANG=en_US seq 140 20 200))
L=${Ls[$SLURM_ARRAY_TASK_ID]}
T=${Ts[$SLURM_ARRAY_TASK_ID]}
tcor=${tcors[$SLURM_ARRAY_TASK_ID]}
./target/release/monte-carlo-cdt -m -t $T -l $L -n 100 -r 0.4 -b 200 -p $tcor -o "./data"