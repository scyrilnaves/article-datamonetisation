#!/bin/bash

./delete-buildcontainer.sh
sleep 10
./genContainerbuild.sh >buildcontainer-kube.yaml
./deploy-buildcontainer.sh

echo "Done"
