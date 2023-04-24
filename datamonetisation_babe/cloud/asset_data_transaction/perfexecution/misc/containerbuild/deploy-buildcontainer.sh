#!/bin/bash

cd ./rancher-v2.4.10/
#./login.sh $1
./login.sh

echo "Load Deployments"

#apply main yaml:
./rancher kubectl -n substrate-net apply -f ../buildcontainer-kube.yaml --validate=false

echo "Done"
