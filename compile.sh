#!/bin/bash
cd ~/Documents/monte-carlo-CDT
git fetch
git reset --hard HEAD
git merge origin/correlation_test
cargo build --release