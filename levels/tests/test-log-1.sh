#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=turbulator-feere-reinclined
level_title=log-1

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
if [[ $(git log -n 3 | tail -n 1) == *"insatiably-skyjackers-program"*]]; then 
    # manual pass (no push)
    exit 0 
else
    # manual fail (no push)
    exit 1
fi
