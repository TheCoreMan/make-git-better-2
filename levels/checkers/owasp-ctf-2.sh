#!/bin/bash

source $(dirname $0)/checkers-lib.sh

read old new ref < /dev/stdin

dump_dir=$(dump-commit-to-directory $new)
script_path=".build/build.sh"
flag_user_name="flagger_the_second"

# Build system verification stages

commit_message=$(get_commit_message $new)
commit_author_name=$(get_commit_author_name $new)
commit_author_email=$(get_commit_author_email $new)
commit_timezone=$(get_commit_timzone $new)
build_step=0

echo_build_step() {
    echo "[$build_step] [-] $1"
}

echo_build_pass() {
    echo "[$build_step] [V] $1"
    echo "[$build_step] [V]     #$build_step passed! (* ^ ω ^)"
    ((build_step++))
}

echo_build_fail() {
    echo "[$build_step] [X] $1"
    echo "[$build_step] [X] (╮°-°)╮┳━━┳ ( ╯°□°)╯ ┻━━┻"
    exit 1
}

echo
echo "Welcome to the awesome build system."
echo
echo " -=- Legend -=- "
echo "[BUILD STEP] [STATUS] {message from build system}"
echo "[n] [-]     Step n is 'in progress'."
echo "[n] [V]     Step n passed, will continue to next step."
echo "[n] [X]     Step n failed, will exit the build."
echo

echo_build_step "Initializing build system..."
echo_build_pass "Build system initialized."

echo_build_step "Checking authorization by name..."
echo_build_step "Found name: $commit_author_name"
if [[ ! $commit_author_name =~ ^(Johnny Cash|Glen Campbell|Willie Nelson)$ ]]; then
    echo_build_fail "Authorization failed! Non-authorized commit author name."
else
    echo_build_pass "Authorization by name passed."
fi

echo_build_step "Checking authorization by email..."
echo_build_step "Found email: $commit_author_email"
if [[ ! ${commit_author_email,,} =~ ^(johnnycash@build.system|glencampbell@build.system|willienelson@build.system)$ ]]; then
    echo_build_fail "Authorization failed! Non-authorized commit author email."
else
    echo_build_pass "Authorization by email passed."
fi

echo_build_step "Checking authorized geolocation..."
echo_build_step "Found timezone: $commit_timezone"
if [ $commit_timezone != "-1200" ]; then
    echo_build_fail "Geolocation auth failed! Non-authorized timezone. Please travel to Baker Island, Howland Island or anywhere within the IDLW, set your clock, and commit from there. Make sure to pack enough food, drink and other supplies for the journey."
else
    echo_build_pass "Authorization by timezone passed."
fi

# TODO - copy flag and read it somehow

pushd $dump_dir
    echo_build_step "Creating temporary build directory for build resources..."
    export TMP_BUILD_SYSTEM_DIR=$(mktemp -d)
    chmod 0777 $TMP_BUILD_SYSTEM_DIR
    echo_build_pass "Created temporary directory for build resources in $TMP_BUILD_SYSTEM_DIR which can be accessed via an environment variable."

    echo_build_step "Copying build resources to temporary build directory..."
    sudo -u $flag_user_name cp --verbose /flag.txt $TMP_BUILD_SYSTEM_DIR/flag.txt
    sudo chown --verbose build_system:build_system $TMP_BUILD_SYSTEM_DIR/flag.txt
    echo_build_pass "Copied and chmod-ed build resources."

    echo_build_step "Locating build script in $script_path..."
    if [ ! -f $script_path ]; then
        echo_build_fail "$script_path not found!";
    fi
    echo_build_pass "Build script located!"

    echo_build_step "Giving .build directory exec permissions..."
    chmod --verbose --recursive 777 .build
    chmod --recursive 777 $dump_dir
    echo_build_pass ".build directory chmod-ed"

    echo_build_step "Running '$script_path' using 'bash' as user 'build_system'"
    chmod 0777 $script_path
    sudo -u build_system env TMP_BUILD_SYSTEM_DIR=$TMP_BUILD_SYSTEM_DIR bash $script_path
    echo_build_pass "Done running build script."

    echo_build_step "Deleting temporary build directory..."
    rm -rf --verbose $TMP_BUILD_SYSTEM_DIR
    echo_build_pass "Deleted temporary build directory."

    echo_build_step "Deploying output to @build.system cloud..."
    echo_build_fail "@build.system cloud doesn't exist yet. Exiting build."
popd

# level info
## title owasp-ctf-2 
## branch aghastness-subhead-cyrtometer -> That means the tag is aghastness-subhead-cyrtometer-tag
