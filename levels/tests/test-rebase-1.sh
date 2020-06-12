#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=parallelizing-barnhardtite-base
level_title=rebase-1

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
git checkout parallelizing-barnhardtite-topic
chmod +x add_resources.sh
./add_resources.sh
git add runme_resources
git rm add_resources.sh -f
git commit -m "added resources"
git rebase parallelizing-barnhardtite-base parallelizing-barnhardtite-topic
git checkout parallelizing-barnhardtite-base
git merge parallelizing-barnhardtite-topic

git push > push_result 2>&1

check_results
