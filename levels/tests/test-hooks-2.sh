#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=cyprus-akees-metope
level_title=hooks-2

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit

git push > push_result 2>&1

check_results
