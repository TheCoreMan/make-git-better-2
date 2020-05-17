#!/bin/bash

read old new ref < /dev/stdin

branch_name=$(echo $ref | awk 'BEGIN \{ FS = "/" } ; \{ print $NF }')

case $branch_name in
{{ for level in levels }}{level.branch})
    echo $old $new $ref | {level.solution_checker}
    echo "solved! the flags are..."
    {{ for levelflag in level.flags }}
    echo {levelflag}
    {{ endfor }}
    ;;
{{ endfor }}esac
