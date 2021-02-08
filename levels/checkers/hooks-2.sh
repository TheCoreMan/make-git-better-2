#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    # Check files here
popd

# level info
## title hooks-2 
## branch cyprus-akees-metope -> That means the tag is cyprus-akees-metope-tag

commit_amount=$( git log cyprus-akees-metope-tag..$new --oneline | wc -l )
if (( $commit_amount < 1 )); then
    reject-solution "Not enough commits: saw "$commit_amount" commits, expected 1 or more.";
fi

if git log --format=%B -n 1 $new |  grep -iq ".*git is awesome.*"; 
then
    echo "Message '$(git log --format=%B -n 1 $new)' is good"
else
    reject-solution "Commit message '$(git log --format=%B -n 1 $new)' doesn't contain 'git is awesome', which is what the commit-msg hook tried to enforce. Try again.";
fi
