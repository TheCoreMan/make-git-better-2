#!/bin/bash

current_branch = `git rev-parse --abbrev-ref HEAD`

case $current_branch in
{{ for level in levels }}{level.branch})
    {level.solution_checker}
    ;;
{{ endfor }}esac