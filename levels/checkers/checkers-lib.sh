#!/bin/bash

# This file has common functions for all the checkers and the pre-receive hook file.

# Pass rejection reason as first parameter.
# Example usage:
# if [ ! -f runme.py ];
#     then reject-solution "runme.py is missing.";
# fi
reject-solution () {
	message=$1
	echo "Solution rejected. Reason: "
	echo $message
	exit 1
}

# Used to dump commit contents to a directory. Returns the path. 
# Example usage: 
# dump_dir=$(dump-commit-to-directory $new)
# pushd $dump_dir
dump-commit-to-directory () {
	commit_hash=$1
	temp_dir=$(mktemp -d)
	(git archive $commit_hash | tar -xf - -C $temp_dir) &>/dev/null
	echo $temp_dir
}

# Override pushd and popd to be silent, use like normal pushd and popd
pushd () {
    command pushd "$@" > /dev/null
}

popd () {
    command popd "$@" > /dev/null
}

print_flag_icon() {
flag_icon="
        ()__
        ||  Z__
        ||  |   Z____
        ||  |   |    |
        ||  |   |    |
        ||__|   |    |
        ||  /___|    |
        ||      /____/
        ||
        ||
        ||
        /\\                
    ___/  \\___
"

	echo "$flag_icon"
}

print_seperator() {
	echo
	echo "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"
	echo
}


# Pass git pointer (branch, tag, commit)
# Example usage: target_commit=$(get_commit_of steek-shabandar-taenifuge-tag)
get_commit_of() {
    echo $(git rev-list -n 1 $1)
}

how_many_parents() {
    echo $(git cat-file -p $1 | grep parent | wc -l)
}
