#!/bin/bash

setup_repo_for_test() {
    test_dir=test_$(date +%Y%m%d_%H%M%S)
    mkdir $test_dir
    pushd $test_dir
    git clone gamemaster@localhost:~/ctf-repo
    pushd ctf-repo
    echo Finished setup, currently in $(pwd)
}

# pass results in a file called push_result
check_results() {
    if [ $(grep "You won" push_result | wc -l) -gt 0 ]; 
    then
        git reset --hard 
        git clean -f -d
        exit 0
    else 
        git reset --hard
        git clean -f -d
        exit 1
    fi
}

teardown() {
    popd
    popd
}
