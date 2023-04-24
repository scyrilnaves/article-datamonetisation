#!/bin/bash

echo "MAIN FINAL compiling all results"

# Enter starttime endtime startblockhash

./export_all_data.sh $1 $2

echo "Finish Export Data"

./finalstats.js $3

echo "END FINAL compiling all results"
