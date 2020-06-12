#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    # Check files here
popd

# level info
## title hooks-2 
## branch cyprus-akees-metope -> That means the tag is cyprus-akees-metope-tag

# Check how many commits the user needed - shouldn't be more than 3!
commit_amount=$( git log cyprus-akees-metope-tag..$ref --oneline | wc -l )
if [ $commit_amount -gt 0 ];
    then bad "Not enough commits: expected at least "$commit_amount" commits";
fi
