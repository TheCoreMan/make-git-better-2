function f_retag {
    cur_branch=$( git rev-parse --abbrev-ref HEAD )
    tag_suffix="-tag"
    tag_name=$cur_branch$tag_suffix
    git --no-pager log --oneline --graph -n 10 --decorate
    echo "Current branch is ${cur_branch} and the tag name will be ${tag_name}."
    read -q "answer?Is this OK? (y/n)?" -n 1
    echo
    case ${answer:0:1} in
            y|Y )
                    echo "Good, creating tag and pushing it"
                    ;;
            * )
                    echo "No"
                    return 1
                    ;;
    esac

    # actually do the thing
    git tag -f $tag_name
    git push --tags -f
}

alias gretag=f_retag
