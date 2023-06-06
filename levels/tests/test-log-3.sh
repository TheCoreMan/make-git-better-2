#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=originates-anagyrine-untolerative
level_title=log-3

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
if [[ $(git --no-pager log -S"The flag is" --oneline) == *"commit 313"* ]]; then
    # manual pass (no push)
    exit 0 
else
    # manual fail (no push)
    exit 1
fi
