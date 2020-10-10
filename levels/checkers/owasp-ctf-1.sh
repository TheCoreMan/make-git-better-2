#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)
script_path="capture_the_flag.py"
flag_user_name="flagger"

pushd $dump_dir
    echo "Right now I'm running as $(whoami) in $(pwd)"

    if [ ! -f $script_path ]; then
        reject-solution "$script_path is missing.";
    else
        echo "$script_path found, attempting to execute it as $flag_user_name"
        chmod 0777 $dump_dir
        chmod 0766 $script_path
        ls -la .
    fi

    echo -n "Right now Im running as "
    sudo -u $flag_user_name whoami
    sudo -u $flag_user_name python3 $script_path

    echo "Right now I'm running as $(whoami)"

    echo "I've attempted to run the script. Bye bye now."
    exit 1
popd

# level info
## title owasp-ctf-1 
## branch headmistresses-tiptoes-bezzled -> That means the tag is headmistresses-tiptoes-bezzled-tag
