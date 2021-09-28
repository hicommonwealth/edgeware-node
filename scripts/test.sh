#!/bin/bash
SCRIPTDIR=$PWD
for d in $(ls -d ./pallets/*/) ; do
    cd "$SCRIPTDIR/$d" && cargo test
done
