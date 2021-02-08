#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=reappraise-veratroyl-garfishes
level_title=merge-5

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
git merge origin/vicomtesses-apery-vanillin || true  # will conflict
# resolve merge by taking the code from one branch and manually putting it in the other branch's rename
cat should_rename_this.py > scripts/print_cool_stuff.py
# take the new name (with the old content)
git add scripts/print_cool_stuff.py
# remove all other files
git rm runme.py should_rename_this.py
git commit -m "Resolved conflict semi-manually"

git push > push_result 2>&1

check_results
