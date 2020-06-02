# make-git-better-2

Git CTF 🚩 but good this time.

- [make-git-better-2](#make-git-better-2)
  - [Dependencies](#dependencies)
  - [Build](#build)
    - [How to build the challenge Docker](#how-to-build-the-challenge-docker)
      - [Create the hook script](#create-the-hook-script)
        - [powershell](#powershell)
        - [sh](#sh)
      - [Build and run docker image](#build-and-run-docker-image)
        - [Build docker](#build-docker)
        - [Run docker](#run-docker)
        - [Useful oneliner](#useful-oneliner)
        - [Connect to the running instance](#connect-to-the-running-instance)
    - [How to build the web content](#how-to-build-the-web-content)
      - [Build the level browser](#build-the-level-browser)
    - [Set up docker-tcp-switchboard](#set-up-docker-tcp-switchboard)
  - [Test](#test)
    - [Unit tests](#unit-tests)
    - [Test levels](#test-levels)
  - [Develop](#develop)
    - [Add a new stage](#add-a-new-stage)

## Dependencies

- Rust
- Docker
- Python 3.6 (for docker TCP switchboard)

## Build

### How to build the challenge Docker

#### Create the hook script

`cd` to the `scripts` directory.

##### powershell

```powershell
cargo run --bin generate-pre-receive-hook -- --verbose ..\levels\game-config.toml .\src\bin\templates\hook.tmpl
```

##### sh

```sh
cargo run --bin generate-pre-receive-hook -- --verbose ../levels/game-config.toml src/bin/templates/hook.tmpl
```

#### Build and run docker image

##### Build docker

```sh
docker build --tag mgb:0.1 --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S) .
```

##### Run docker

```sh
docker run --detach --name mgbtest --publish 7777:22 mgb:0.1
```

##### Useful oneliner

```sh
docker rm -f mgbtest && docker build --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S%z) --tag mgb:0.1 . && docker run --detach --name mgbtest --publish 7777:22 mgb:0.1
```

##### Connect to the running instance

Password is `player`.

```sh
ssh player@localhost -p 7777
```

### How to build the web content

#### Build the level browser

```sh
cargo run --bin generate-levels-graph -- -v ../levels/game-config.toml src/bin/templates/graph.tmpl
```

### Set up docker-tcp-switchboard

*Only relevant for the game server, no need to do this for local build*.

```sh
git clone https://github.com/OverTheWireOrg/docker-tcp-switchboard.git
# install deps and then
touch /var/log/docker-tcp-switchboard.log
chmod a+w /var/log/docker-tcp-switchboard.log
```

Copy `build/docker-tcp-switchboard.conf` to `/etc/docker-tcp-switchboard.conf`. Finally, run `python3 docker-tcp-switchboard.py`.

## Test

### Unit tests

```sh
cd scripts
cargo test
```

### Test levels

Login as user `tester` to the built Docker and then:

```sh
cd tests
./run_all_tests.sh 2>&1 | grep "TESTLOG"
```

Should print something like:

```sh
tester@9fd701bf937c:~/tests
> ./run_all_tests.sh 2>&1 | grep "TESTLOG"
TESTLOG: Testing /home/tester/tests/test-basic-1.sh
TESTLOG: testing level basic-1 branch scorpion-treenware-gestatory
TESTLOG: Test /home/tester/tests/test-basic-1.sh passed
TESTLOG: Testing /home/tester/tests/test-basic-2.sh
TESTLOG: testing level basic-2 branch sidespins-areae-regalio
TESTLOG: Test /home/tester/tests/test-basic-2.sh passed
TESTLOG: Testing /home/tester/tests/test-merge-1.sh
TESTLOG: testing level merge-1 branch macrochiropteran-jupon-lutecium
TESTLOG: Test /home/tester/tests/test-merge-1.sh passed
TESTLOG: Testing /home/tester/tests/test-merge-2.sh
TESTLOG: testing level merge-2 branch poseuse-citronwood-manganese
TESTLOG: Test /home/tester/tests/test-merge-2.sh passed
TESTLOG: Testing /home/tester/tests/test-start-here.sh
TESTLOG: testing level start-here branch start-here
TESTLOG: Test /home/tester/tests/test-start-here.sh passed
TESTLOG: Out of 5 tests, 5 passed and 0 failed.
```

## Develop

### Add a new stage

```powershell powershell
cargo run --bin generate-new-level -- ..\levels\game-config.toml .\src\bin\templates\level_checker.tmpl .\src\bin\templates\level_test.tmpl .\src\bin\templates\level_page.tmpl .\src\bin\resources\words_alpha.txt ..\levels\ -v
```
