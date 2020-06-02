#!/bin/bash

source $(dirname $0)/common.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    if [ ! -f runme.py ];
        then reject-solution "runme.py is missing.";
    fi

    echo "Trying to execute the script ./runme.py..."
    echo

    # Will raise if not merged
    ./runme.py
popd

# Check that merge was a commit merge.
new_commit=$(get_commit_of $new)
target_commit=$(get_commit_of steek-shabandar-taenifuge-tag)
if [ "$new_commit" == "$target_commit" ]; then
    reject-solution "Merge shouldn't have been a fast-forward merge. Try again."
fi

how_many_parents=$(how_many_parents $new_commit)
if [ $how_many_parents -ne 2 ]; then
    reject-solution "Pushed commit isn't a merge commit! Saw only $how_many_parents for this commit, expected 2. Try again."
fi

log_n=4
echo "git log of last $log_n commits..."
git log --oneline --graph -n $log_n $new_commit
# level info
## title merge-2 
## branch poseuse-citronwood-manganese -> That means the tag is poseuse-citronwood-manganese-tag
