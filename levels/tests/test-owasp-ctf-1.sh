#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=headmistresses-tiptoes-bezzled
level_title=owasp-ctf-1

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
mv capture_ze_flag.py capture_the_flag.py
git add --all
git commit -m "moved"
sed -i 's/\[REDACTED\]/\/etc\/owasp\/flags\/flag.txt/g' capture_the_flag.py
git add --all
git commit -m "fixed path"

git push > push_result 2>&1

# check_results
if [ $(grep "AppSec_IL" push_result | wc -l) -gt 0 ]; then
    git reset --hard > /dev/null 2>&1
    git clean -f -d > /dev/null 2>&1
    exit 0
else 
    git reset --hard > /dev/null 2>&1
    git clean -f -d > /dev/null 2>&1
    exit 1
fi
