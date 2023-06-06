#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref </dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
# Check files here
if [ ! -f first.md ]; then
    reject-solution "first.md is missing."
fi

if [ ! -f second.md ]; then
    reject-solution "second.md is missing."
fi

if [ ! -f third.md ]; then
    reject-solution "third.md is missing."
fi

# Check history here

popd

# level info
## title rebase-2
## branch downfalling-bumbled-sootiness -> That means the tag is downfalling-bumbled-sootiness-tag

# Check the order. It should be first, second, third.
FIRST_COMMIT_MESSAGE=$(git log --pretty=format:%s HEAD~2 -n 1)
SECOND_COMMIT_MESSAGE=$(git log --pretty=format:%s HEAD~1 -n 1)
THIRD_COMMIT_MESSAGE=$(git log --pretty=format:%s HEAD -n 1)
if grep -q "$FIRST_COMMIT_MESSAGE" "first"; then
    reject-solution "The first commit should be \"Who's on first\", but saw $FIRST_COMMIT_MESSAGE."
fi
if grep -q "$SECOND_COMMIT_MESSAGE" "second"; then
    reject-solution "The second commit should be \"What's on second\", but saw $SECOND_COMMIT_MESSAGE."
fi
if grep -q "$THIRD_COMMIT_MESSAGE" "third"; then
    reject-solution "The third commit should be \"I don't know's on third\", but saw $THIRD_COMMIT_MESSAGE."
fi

log_n=5
echo "git log of last $log_n commits..."
git log --decorate --oneline --graph -n $log_n "$new"
