#!/bin/bash
cp .dockerignore .dockerignore.original
ln -fs .dockerignore.remotecargo .dockerignore
docker build -t cwl/edgeware -f remotecargo.dockerfile .
docker run -it cwl/edgeware --dev
