#!/bin/bash

source $(dirname $0)/common.sh

level_branch=poseuse-citronwood-manganese
level_title=merge-2

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
git merge origin/cannoneer-dephlegm-holoptychius -m "test merge 2"

git push > push_result 2>&1

check_results
