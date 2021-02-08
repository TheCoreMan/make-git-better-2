#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    # Check files here
    if grep -q ".*Celebrimbor o Eregion teithant i thiw hin.*" password_file.txt; 
    then echo "Password matches, friend."
    else reject-solution "Password doesn't match! Are you sure you've merged the correct password?"
    fi
popd

# level info
## title remote-1 
## branch geared-tidal-consolidated -> That means the tag is geared-tidal-consolidated-tag

if [[ $(git log --oneline -n 5 $new) != *"443ad11"* ]]; then
    reject-solution "The commit from the remote repo is missing in this history. Try again and make sure you merge the changes from there."
fi
