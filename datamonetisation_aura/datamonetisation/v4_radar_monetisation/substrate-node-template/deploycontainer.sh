#!/bin/sh

# DOCKER PART

echo "REMOVE-CONTAINER"
docker rmi --force cyrilthese/datamonetisation_aura:latest

echo "CONTAINER-BUILD"
docker build -f dockerfile_node_binary -t cyrilthese/datamonetisation_aura:latest --no-cache .

echo "CONTAINER-PUSH"
docker push cyrilthese/datamonetisation_aura:latest
