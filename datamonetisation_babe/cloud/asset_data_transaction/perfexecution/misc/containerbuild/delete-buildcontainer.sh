#!/bin/bash
cd ./rancher-v2.4.10/

#$1 is the rancher login token

#./login.sh $1
./login.sh

echo "Delete Deployments"
./rancher kubectl -n substrate-net delete deployments --all

echo "Delete Services"
./rancher kubectl -n substrate-net delete services --all

# ./rancher kubectl -n substrate-net delete pv --all

echo "Done"
