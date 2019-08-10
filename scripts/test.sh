#!/bin/bash
SCRIPTDIR=$PWD
for d in $(ls -d ./modules/*/) ; do
    cd "$SCRIPTDIR/$d" && cargo test
done
