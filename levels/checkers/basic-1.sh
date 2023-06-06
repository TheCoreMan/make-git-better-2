#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    if [ -f deleteme.txt ];
        then reject-solution "deleteme.txt is still here. Try again.";
    fi
    if [ ! -f dontdeleteme.txt ];
        then reject-solution "dontdeleteme.txt was deleted! Try again.";
    fi
popd

commit_amount=$( git log scorpion-treenware-gestatory-tag..$new --oneline | wc -l )
if [ $commit_amount -ne 1 ];
    then reject-solution "Should have been solved using a single commit, but I've found ${commit_amount} commits in the log.";
fi
