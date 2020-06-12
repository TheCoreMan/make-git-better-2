#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

reject-solution "Shouldn't push in this level."

# level info
## title tag-2 
## branch individually-nonintroversive-chalcomancy -> That means the tag is individually-nonintroversive-chalcomancy-tag
