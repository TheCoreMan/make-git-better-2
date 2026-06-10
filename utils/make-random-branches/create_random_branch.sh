#!/bin/bash

set -e 
set -x

if [ "$(git rev-parse --is-inside-work-tree 2>/dev/null)" != "true" ]; then
	echo "not in git repo"
	exit 1
fi


echo "Creating new noise branch"

branch_name=$(./get-branch-name ./words_alpha.txt)

echo "branch name: "$branch_name

git status

git checkout -b $branch_name

head -c 100 </dev/urandom >file_$branch_name

git add file_$branch_name
git commit -m "Committing $branch_name"
git tag "$branch_name-tag"
git push -u origin $branch_name
git push origin --tags

