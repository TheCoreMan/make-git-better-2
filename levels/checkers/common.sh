#!/bin/bash

reject-solution () {
	message=$1
	echo "Solution rejected. Reason: "
	echo $message
	exit 1
}

dump-commit-to-directory () {
	commit_hash=$1
	temp_dir=$(mktemp -d)
	(git archive $commit_hash | tar -xf - -C $temp_dir) &>/dev/null
	echo $temp_dir
}

# Override pushd and popd to be silent
pushd () {
    command pushd "$@" > /dev/null
}

popd () {
    command popd "$@" > /dev/null
}
