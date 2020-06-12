#!/bin/bash

source $(dirname $0)/tests-lib.sh

level_branch=individually-nonintroversive-chalcomancy
level_title=tag-2

test_log "testing level $level_title branch $level_branch"

git checkout $level_branch
git clean -f -d

# PUT TEST CODE HERE, like git add + git commit
if [[ $(git show $(git show-ref --tags -d | grep ^$(git --no-pager log --pretty=%P -n 1) | sed -e 's,.* refs/tags/,,' -e 's/\^{}//')) == *"nine-botchy-remarker (final)"* ]]; then
    # manual pass (no push)
    exit 0 
else
    # manual fail (no push)
    exit 1
fi
