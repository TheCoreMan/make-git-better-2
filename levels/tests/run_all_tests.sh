#!/bin/bash

tests_dir=$(realpath $(dirname $0))

source $tests_dir/common.sh

setup_repo_for_test

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

for test_script in $tests_dir/test-*; do
    echo Testing $test_script
    if bash $test_script; 
    then
        echo -e "Test ${test_script} ${GREEN}passed${NC}"
    else 
        echo -e "Test ${test_script} ${RED}failed${NC}"
    fi
done

teardown
