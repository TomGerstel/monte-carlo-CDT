#!/bin/bash
#SBATCH --partition=hefstud
#SBATCH --output=./cluster_output/std_%A_%a.txt
#SBATCH --mem=100M
#SBATCH --time=3:00:00
#SBATCH --array=0-3%4
cd ~/Documents/monte-carlo-CDT
index={0..4}
i=${index[$SLURM_ARRAY_TASK_ID]}
L=$((15+5*$i))
T=$((20*$L))
tcor=$((4*$L))
./target/release/monte-carlo-cdt -m -t $T -l $L -n 100 -r 0.4 -b 200 -p $tcor -o "./data"