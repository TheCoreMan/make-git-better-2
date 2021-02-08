#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=sidespins-areae-regalio
level_title=basic-2

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# Actual test code
mkdir newdir
touch newdir/f1 newdir/f2
git add newdir
git commit -m "Testing basic-2."

git push > push_result 2>&1

check_results
