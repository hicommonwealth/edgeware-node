#!/bin/bash
cp .dockerignore .dockerignore.original
ln -fs .dockerignore.remoteCargo .dockerignore
docker build -t cwl/edgeware -f RemoteCargo.dockerfile .
docker run -it cwl/edgeware --dev
