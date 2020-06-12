#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

# level info
## title tag-1 
## branch redamage-bundh-passerina -> That means the tag is redamage-bundh-passerina-tag

echo "testing"
echo "old $old"
echo "new $new"
echo "ref $ref"
echo "finished"

exit 1
