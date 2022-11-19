#! /bin/bash

NETWORK_ALIAS=$1

for f in ./tests/tests/*.sh; do
    bash "$f" $NETWORK_ALIAS
done