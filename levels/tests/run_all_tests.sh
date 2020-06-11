#!/bin/bash

tests_dir=$(realpath $(dirname $0))

source $tests_dir/tests-lib.sh

run_test_suite() {
    setup_repo_for_test

    total=$(find $tests_dir/test-* | wc -l)
    passed=0
    failed=0

    for test_script in $tests_dir/test-*; do
        test_log "Testing $test_script"
        if bash $test_script; 
        then
            test_log "Test ${test_script} ${GREEN}passed${NC}"
            passed=$((passed + 1))
        else 
            test_log "Test ${test_script} ${RED}failed${NC}"
            failed=$((failed + 1))
        fi
    done

    test_log "Out of ${BLUE}$total tests${NC}, ${GREEN}$passed passed${NC} and ${RED}$failed failed${NC}."

    teardown
}

if [[ $1 == "-v" ]]; then
    run_test_suite
else run_test_suite 2>&1 | grep "TESTLOG"
fi
