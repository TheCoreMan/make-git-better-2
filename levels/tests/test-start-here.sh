#!/bin/bash

source $(dirname $0)/common.sh

level_branch=start-here
level_title=start-here

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

touch alice.txt bob.txt
git add *
git commit -m "Testing start-here."

git push > push_result 2>&1

check_results
