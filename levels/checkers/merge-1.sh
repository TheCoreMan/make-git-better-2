#!/bin/bash

source $(dirname $0)/checkers-lib.sh

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

# Check that merge was fast forward merge.
new_commit=$(get_commit_of $new)
target_commit=$(get_commit_of steek-shabandar-taenifuge-tag)
if [ "$new_commit" != "$target_commit" ]; then
    reject-solution "Merge should have been a fast-forward merge. Try again."
fi

# level info
## title merge-1 
## branch macrochiropteran-jupon-lutecium -> That means the tag is macrochiropteran-jupon-lutecium-tag
