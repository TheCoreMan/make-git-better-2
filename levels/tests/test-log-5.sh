#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=pamphletary-harnessing-petticoaterie
level_title=log-5

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
exit 0
