#!/bin/bash

source $(dirname $0)/common.sh

level_branch=macrochiropteran-jupon-lutecium
level_title=merge-1

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
git merge origin/steek-shabandar-taenifuge

git push > push_result 2>&1

check_results
