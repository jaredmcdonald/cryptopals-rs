#!/bin/bash

FORMATTED_N=$(printf "%02d" $1)

# create the file and its export
echo "pub fn run_$1() {}" > src/exercise_$FORMATTED_N.rs

# add imports etc to main.rs
PREVIOUS_N=$(echo "$1 - 1" | bc -l)
FORMATTED_PREVIOUS_N=$(printf "%02d" $PREVIOUS_N)

gsed -i "/mod exercise_"$FORMATTED_PREVIOUS_N"/a mod exercise_"$FORMATTED_N";" src/main.rs
gsed -i "/use exercise_"$FORMATTED_PREVIOUS_N"/a use exercise_"$FORMATTED_N"::run_"$FORMATTED_N";" src/main.rs
gsed -i "/Ok("$FORMATTED_PREVIOUS_N")/a \            Ok("$FORMATTED_N") => run_"$FORMATTED_N"()," src/main.rs
