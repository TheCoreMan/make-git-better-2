#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    if [ ! -f scripts/print_cool_stuff.py ];
        then reject-solution "scripts/print_cool_stuff.py is missing.";
    fi

    if [ -f runme.py ];
        then reject-solution "runme.py is still here!";
    fi

    if [ -f should_rename_this.py ];
        then reject-solution "should_rename_this.py is still here!";
    fi

    echo "Trying to execute the script..."
    echo
    
    ./scripts/print_cool_stuff.py

    if [[ $(./scripts/print_cool_stuff.py) != *"You're cool"* ]] ; then
        reject-solution "Seems like the merge didn't go right - the script should print the words 'You're cool'."
    fi
    if [[ $(./scripts/print_cool_stuff.py) != *"The current git version you're playing with is"* ]] ; then
        reject-solution "Seems like the merge didn't go right - the script should print the current git version."
    fi
    if [[ $(./scripts/print_cool_stuff.py) != *"Today I have grown taller from walking with the trees"* ]] ; then
        reject-solution "Seems like the merge didn't go right - the script should print a quote about trees."
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
## title merge-5 
## branch reappraise-veratroyl-garfishes -> That means the tag is reappraise-veratroyl-garfishes-tag
