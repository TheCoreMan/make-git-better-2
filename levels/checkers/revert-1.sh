#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    if [ ! -f runme.py ];
        then reject-solution "runme.py is missing.";
    fi

    echo "Trying to execute the script ./runme.py..."
    echo
    
    # Will raise the following if revert hasn't happened:
    #   File "./runme.py", line 5
    # Oops - didn't mean to commit this line
    #                                       ^
    # SyntaxError: EOL while scanning string literal
    ./runme.py
popd

commit_message=$(get_commit_message $new)

if [[ $commit_message != "Revert"*"Oops - didn't mean to commit this."*"This reverts commit 1948a60c6ee8098fce8524b48426ec06b9938d58."* ]]; then
    reject-solution "I expected the commit message to be the default one of a revert commit, but the commit message is: $commit_message. Use the default commit message of git revert."
fi

# level info
## title revert-1 
## branch lomentaceous-mididae-hexadecane -> That means the tag is lomentaceous-mididae-hexadecane-tag
