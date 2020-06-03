#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=belialist-interlaying-mize
level_title=log-4

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
if [[ $(git --no-pager log HEAD~199 --oneline) == *"pamphletary-harnessing-petticoaterie"* ]]; then
    # manual pass (no push)
    exit 0 
else
    # manual fail (no push)
    exit 1
fi
