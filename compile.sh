#!/bin/bash
cd ~/Documents/monte-carlo-CDT
git fetch
git reset --hard HEAD
git merge origin/master
cargo build --release