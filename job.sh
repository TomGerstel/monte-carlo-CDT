#!/bin/bash
#SBATCH --partition=hefstud
#SBATCH --output=std_%A_%a.txt
#SBATCH --mem=100M
#SBATCH --time=2:00:00
cd ~/Documents/monte-carlo-CDT
./target/release/monte-carlo-cdt