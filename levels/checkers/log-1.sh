#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    # Check files here
popd

reject-solution "Shouldn't push in this level."

# level info
## title log-1 
## branch turbulator-feere-reinclined -> That means the tag is turbulator-feere-reinclined-tag
