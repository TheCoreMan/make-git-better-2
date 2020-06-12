#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    how_many_files=$(ls add_files_here -1 | wc -l)

    if [ $how_many_files -lt 99 ]; 
        then bad "Not enough files: only "$how_many_files", need more than 99"
    fi
popd

# level info
## title hooks-1 
## branch hands-trooshlach-nongassy -> That means the tag is hands-trooshlach-nongassy-tag

# Check how many commits the user needed - shouldn't be more than 3!
commit_amount=$( git log hands-trooshlach-nongassy-tag..$ref --oneline | wc -l )
if [ $commit_amount -gt 3 ];
    then bad "Too many commits! Took "$commit_amount" commits";
fi
