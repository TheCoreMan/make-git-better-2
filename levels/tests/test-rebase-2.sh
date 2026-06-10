#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=downfalling-bumbled-sootiness
level_title=rebase-2

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
# This level tests interactive rebasing. Since this
# is a non-interactive test, we'll have to use
# cherry-picking instead. :(

# See https://gist.github.com/textarcana/1306223
last_3_commits_json=$(
    git log -n 3 \
        --pretty=format:'{%n  "commit": "%H",%n  "author": "%aN <%aE>",%n  "date": "%ad",%n  "message": "%f"%n},' \
        $@ |
        perl -pe 'BEGIN{print "["}; END{print "]\n"}' |
        perl -pe 's/},]/}]/'
)

# Get the relevant IDs (for cherry picking later)
first_commit_id=$(echo "$last_3_commits_json" | jq -r '.[] | select( .message | contains("first")) | .commit')
second_commit_id=$(echo "$last_3_commits_json" | jq -r '.[] | select( .message | contains("second")) | .commit')
third_commit_id=$(echo "$last_3_commits_json" | jq -r '.[] | select( .message | contains("third")) | .commit')

# Reset to the commit one before these three commits
git reset --hard HEAD~3
# Cherry pick the commits in the correct order
git cherry-pick "$first_commit_id"
git cherry-pick "$second_commit_id"
git cherry-pick "$third_commit_id"

# For the rebase, we need to force push
git push --force-with-lease >push_result 2>&1

check_results
