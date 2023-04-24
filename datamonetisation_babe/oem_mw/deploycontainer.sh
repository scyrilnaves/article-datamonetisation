#!/bin/sh

# DOCKER PART

echo "REMOVE-CONTAINER"
docker rmi --force cyrilthese/oem-api:latest

echo "CONTAINER-BUILD"
docker build -f oemdocker_springboot -t cyrilthese/oem-api:latest --no-cache .

echo "CONTAINER-PUSH"
docker push cyrilthese/oem-api:latest
