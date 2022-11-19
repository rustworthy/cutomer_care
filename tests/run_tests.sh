#! /bin/bash

for f in ./tests/tests/*.sh; do
    bash "$f"
done