#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=redamage-bundh-passerina
level_title=tag-1

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
git tag my-new-tag

git push --tags > push_result 2>&1

check_results
