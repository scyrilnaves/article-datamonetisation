#!/bin/sh

# DOCKER PART

echo "REMOVE-CONTAINER"
docker rmi --force cyrilthese/datamarket-client:latest

echo "CONTAINER-BUILD"
docker build -f perfdocker -t cyrilthese/datamarket-client:latest --no-cache .

echo "CONTAINER-PUSH"
docker push cyrilthese/datamarket-client:latest
