#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=hands-trooshlach-nongassy
level_title=hooks-1

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
./setup_hooks_stage.sh
for x in {1..100} ; do touch add_files_here/somenewfile$x ; done
git add --all
git commit -m "Added many files - no verifying" --no-verify

git push > push_result 2>&1

check_results
