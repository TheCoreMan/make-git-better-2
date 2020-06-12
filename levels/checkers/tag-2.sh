#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    # Check files here
popd

# level info
## title tag-2 
## branch individually-nonintroversive-chalcomancy -> That means the tag is individually-nonintroversive-chalcomancy-tag
