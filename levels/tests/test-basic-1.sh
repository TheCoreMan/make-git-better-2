#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=scorpion-treenware-gestatory
level_title=basic-1

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# Actual test code
git rm deleteme.txt
git commit -m "Testing basic-1."

git push > push_result 2>&1

check_results
