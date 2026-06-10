#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=multichord-ethicalism-fenestration
level_title=merge-4

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
git merge origin/unconvincing-mesothermal-miles || true  # will conflict
cat $(dirname $0)/resources/merge-4-solution.py > runme.py

git add runme.py
git commit -m "Fixed conflict."

git push > push_result 2>&1

check_results
