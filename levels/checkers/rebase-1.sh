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

    ./runme.py

    if [[ $(./runme.py) != *"The Hobgoblin - a Git Koan by Steve Losh"* ]] || [[ $(./runme.py) != *"Franklin Knight Lane"* ]] || [[ $(./runme.py) != *"It is easy to shoot your foot off with git"* ]] ; then
        reject-solution "Seems like the script isn't printing all the required resources. It should print a Git Koan, a quote about flags, and a quote about git."
    fi

    if [ -f add_resources.sh ];
        then reject-solution "add_resources is still here! You should have deleted it in the topic branch...";
    fi
popd

# level info
## title rebase-1 
## branch parallelizing-barnhardtite-base -> That means the tag is parallelizing-barnhardtite-base-tag

how_many_parents=$(how_many_parents $new)
if [ $how_many_parents -ne 1 ]; then
    reject-solution "Pushed commit is a merge commit! Saw $how_many_parents parents for this commit, expected 1. Try again - the history should be linear."
fi

log_n=10
echo "git log of last $log_n commits..."
git log --decorate --oneline --graph -n $log_n $new
