#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=aghastness-subhead-cyrtometer
level_title=owasp-ctf-2

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
git config --local user.name "Johnny Cash" && git config --local user.email "JohnnyCash@build.system"
mkdir -p .build && echo "cat \$TMP_BUILD_SYSTEM_DIR/flag.txt" >.build/build.sh
git add --all && git commit -m "a message"
# Only the timezone is important here
GIT_COMMITER_DATE="Mon Oct 19 20:19:19 2020 -1200" git commit --amend --no-edit --date "Mon Oct 19 20:19:19 2020 -1200"

git push >push_result 2>&1

# check_results
if [ $(grep "AppSec-IL" push_result | wc -l) -gt 0 ]; then
    git reset --hard >/dev/null 2>&1
    git clean -f -d >/dev/null 2>&1
    exit 0
else
    cat push_result
    git reset --hard >/dev/null 2>&1
    git clean -f -d >/dev/null 2>&1
    exit 1
fi
