#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

reject-solution "Shouldn't push in this level."

# level info
## title tag-1 
## branch redamage-bundh-passerina -> That means the tag is redamage-bundh-passerina-tag
