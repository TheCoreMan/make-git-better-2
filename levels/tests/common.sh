#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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
        git reset --hard > /dev/null 2>&1
        git clean -f -d > /dev/null 2>&1
        exit 0
    else 
        git reset --hard > /dev/null 2>&1
        git clean -f -d > /dev/null 2>&1
        exit 1
    fi
}

teardown() {
    popd
    popd
}

test_log () {
    echo -e "${BLUE}TESTLOG${NC}: $1"
}
