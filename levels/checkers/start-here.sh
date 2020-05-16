#!/bin/bash

source ./common.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd dump_dir
	# Check file existence. 
	if [ ! -f alice.txt ]; 
		then reject-solution "Alice is missing! Try again.";
	fi

	if [ ! -f bob.txt ]; 
		then reject-solution "Bob is missing! Try again.";
	fi
popd

git fetch --tags --quiet  # get all the tags but don't show them to the user
# Check how many commits the user needed - should be two (the user commit + merge commit)!
commit_amount=$( git log start-here-tag..$new --oneline | wc -l )
if [ $commit_amount -ne 1 ];
	then reject-solution "The files should have been added in a single commit, but I've found ${commit_amount} commits in the log. To reset and try again, delete your local start-here branch, checkout the original start-here branch again and try again.";
fi

# We know that there's only one commit in the changes - otherwise it would have failed before.
number_of_files_changed=$( git diff --stat $old $new | grep "files changed" | awk ' {print $1} ' ) 
if [[ $number_of_files_changed -ne 2 ]]
then bad "More than 2 files were changed! Only add alice.txt and bob.txt. Check out the original branch and try again.";
fi

