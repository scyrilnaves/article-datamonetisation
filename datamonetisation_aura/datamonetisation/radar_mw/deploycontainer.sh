#!/bin/sh

# DOCKER PART

echo "REMOVE-CONTAINER"
docker rmi --force cyrilthese/radar-api:latest

echo "CONTAINER-BUILD"
docker build -f radardocker_springboot -t cyrilthese/radar-api:latest --no-cache .

echo "CONTAINER-PUSH"
docker push cyrilthese/radar-api:latest
