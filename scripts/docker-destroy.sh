#!/bin/sh

# Stop all running containers
docker stop $(docker ps -aq)
# Delete all containers
docker rm $(docker ps -a -q)
# Delete all images
docker rmi $(docker images -q)