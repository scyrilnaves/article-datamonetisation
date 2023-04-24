#!/bin/bash

cd ./rancher-v2.4.10/
#./login.sh $1
./login.sh

echo "Load Deployments"
./rancher kubectl -n monitoring apply -f ../monitoring.yaml --validate=false

echo "Done"
