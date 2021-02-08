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
    
    # Will raise the following if conflicts weren't resolved:
    #   File "./runme.py", line 11
    # <<<<<<< HEAD
    # ^
    # IndentationError: expected an indented block
    ./runme.py

    if [[ $(./runme.py) == *"Your cool"* ]] ; then
        reject-solution "Seems like the script still prints the mistake."
    fi
    if [[ $(./runme.py) != *"You're cool"* ]] ; then
        reject-solution "Seems like the merge didn't go right - the script should print the words 'You're cool'."
    fi
popd

how_many_parents=$(how_many_parents $new)
if [ $how_many_parents -ne 2 ]; then
    reject-solution "Pushed commit isn't a merge commit! Saw only $how_many_parents for this commit, expected 2. Try again."
fi

log_n=5
echo "git log of last $log_n commits..."
git log --oneline --graph -n $log_n $new

# level info
## title merge-3 
## branch twee-enfamish-stropharia -> That means the tag is twee-enfamish-stropharia-tag
