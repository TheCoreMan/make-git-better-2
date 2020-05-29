#!/bin/bash

source $(dirname $0)/common.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)

pushd $dump_dir
    if [ ! -d newdir ];
        then reject-solution "Didn't find a directory named `newdir`.";
    fi
    if [ -d newdir ];
        then how_many_files_in_dir=$(ls -1q newdir | wc -l)
        if [ $how_many_files_in_dir -ne 2 ];
            then reject-solution "Needed to find 2 files in the newdir directory, found $($how_many_files_in_dir) instead.";
        fi
    fi
popd

# level info
## title basic-2 
## branch sidespins-areae-regalio -> That means the tag is sidespins-areae-regalio-tag
