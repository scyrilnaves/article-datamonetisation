#!/bin/sh

# DOCKER PART

echo "REMOVE-CONTAINER"
docker rmi --force cyrilthese/datamonetisation_babe:latest

echo "CONTAINER-BUILD"
docker build -f dockerfile_node_binary -t cyrilthese/datamonetisation_babe:latest --no-cache .

echo "CONTAINER-PUSH"
docker push cyrilthese/datamonetisation_babe:latest
