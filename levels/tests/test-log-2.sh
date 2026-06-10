#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=insatiably-skyjackers-program
level_title=log-2

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit

if [[ $(git log --grep "The flag is hidden here") == *"originates-anagyrine-untolerative"* ]]; then 
    # manual pass (no push)
    exit 0 
else
    # manual fail (no push)
    exit 1
fi
