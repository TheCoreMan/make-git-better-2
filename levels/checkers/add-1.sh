#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    # Check files here
popd

# level info
## title add-1 
## branch disallowing-seleucid-quanting -> That means the tag is disallowing-seleucid-quanting-tag
