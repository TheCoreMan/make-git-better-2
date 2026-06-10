# make-git-better-2

Git CTF 🚩 but good this time.

* [Dependencies](#dependencies)
* [Build](#build)
  * [Ansible](#ansible)
  * [How to build the challenge Docker manually](#how-to-build-the-challenge-docker-manually)
    * [Create the hook script](#create-the-hook-script)
      * [powershell](#powershell)
      * [sh](#sh)
    * [Build and run docker image](#build-and-run-docker-image)
      * [Build docker](#build-docker)
      * [Run docker](#run-docker)
      * [Copy ssh key (for outside cloning)](#copy-ssh-key-for-outside-cloning)
      * [Useful oneliner](#useful-oneliner)
      * [Connect to the running instance](#connect-to-the-running-instance)
  * [How to build the web content](#how-to-build-the-web-content)
    * [Build the level browser](#build-the-level-browser)
  * [Set up docker-tcp-switchboard](#set-up-docker-tcp-switchboard)
* [Test](#test)
  * [Unit tests](#unit-tests)
  * [Test levels](#test-levels)
* [Develop](#develop)
  * [Add a new stage](#add-a-new-stage)

## Dependencies

* Rust
* Docker
* Python 3.6 (for docker TCP switchboard)
* Ansible (optional)

## Build

### Ansible

#### EDIT for 2026

I don't know who's paying Python developers to make my life misrable but of course:

1. Python3.8+ is not installable on my machine (amazon linux 1)
2. Ansible won't work with "old" Python versions (3.6, which is what I have)

So to "freeze" everything, run:

```bash
uvx --from 'ansible-core==2.16.*' ansible-playbook -vvv -i hosts2 build.yaml
```

Using Ansible, you can build and deploy the game server from (almost) nothing.

```bash
cd build/ansible
sed -i s/ctf.mrnice.dev/your.server.com/g'' hosts
ansible-playbook -v -i hosts build.yaml
```

Make sure that you have Ansible configured correctly with your SSH keys.
[Here's the docs](https://docs.ansible.com/ansible/latest/inventory_guide/connection_details.html).

> Note: Remember to expose 22 to your IP. If you're like me with AWS EC2, you
> need to add a rule to the security group. Like this:
>
> `aws ec2 authorize-security-group-ingress --group-id PUT_HERE --protocol tcp --port 22 --cidr "$(curl -s https://wtfismyip.com/json | jq -r '.YourFuckingIPAddress')/32"`

#### Why "almost"

If you set up a BRAND new server you have to install the following things ON IT AS WELL
so Ansible will work:

* Git
* Docker
* Rust (needs `cargo`)
* Python (needs `pip`)

For Amazon Linux 2023, here are the commands:

```sh
sudo dnf update -y
sudo dnf install git -y
sudo dnf groupinstall "Development Tools" -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
# test: cargo --version
sudo dnf install -y docker
sudo systemctl start docker
sudo systemctl enable docker
sudo systemctl status docker
sudo usermod -aG docker $USER
newgrp docker
# test: docker run hello-world
sudo dnf install python3 python3-pip -y
# test: python3 --version
pip3 --version
```

### How to build the challenge Docker manually

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
docker build --tag mgb:0.1 --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S) --build-arg OWASP_FLAG_1="AppSec-IL{g1t_d035_P3rM1t_T0_c0mm1T}" --build-arg OWASP_FLAG_2="AppSec-IL{1f_y0u_w4n7_17_c0m3_4nd_917_17}" .
```

##### Run docker

```sh
docker run --detach --name mgbtest --publish 7777:22 mgb:0.1
```

##### Copy ssh key (for outside cloning)

```sh
docker cp mgbtest:/home/player/.ssh/id_rsa ./id_rsa.player
```

##### Useful oneliner

```sh
docker rm -f mgbtest; docker build --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S%z) --build-arg OWASP_FLAG_1="AppSec-IL{g1t_d035_P3rM1t_T0_c0mm1T}" --build-arg OWASP_FLAG_2="AppSec-IL{1f_y0u_w4n7_17_c0m3_4nd_917_17}" --tag mgb:0.1 . && docker run --detach --name mgbtest --publish 7777:22 mgb:0.1
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
./run_all_tests.sh
```

Should print something like:

```sh
tester@ad6145e90679:~/tests
> ./run_all_tests.sh
TESTLOG: Testing /home/tester/tests/test-basic-1.sh
TESTLOG: testing level basic-1 branch scorpion-treenware-gestatory
TESTLOG: Test /home/tester/tests/test-basic-1.sh passed
TESTLOG: Testing /home/tester/tests/test-basic-2.sh
TESTLOG: testing level basic-2 branch sidespins-areae-regalio
TESTLOG: Test /home/tester/tests/test-basic-2.sh passed
TESTLOG: Testing /home/tester/tests/test-log-1.sh
TESTLOG: testing level log-1 branch turbulator-feere-reinclined
TESTLOG: Test /home/tester/tests/test-log-1.sh passed
TESTLOG: Testing /home/tester/tests/test-log-2.sh
TESTLOG: testing level log-2 branch insatiably-skyjackers-program
TESTLOG: Test /home/tester/tests/test-log-2.sh passed
TESTLOG: Testing /home/tester/tests/test-log-3.sh
TESTLOG: testing level log-3 branch originates-anagyrine-untolerative
TESTLOG: Test /home/tester/tests/test-log-3.sh passed
TESTLOG: Testing /home/tester/tests/test-log-4.sh
TESTLOG: testing level log-4 branch belialist-interlaying-mize
TESTLOG: Test /home/tester/tests/test-log-4.sh passed
TESTLOG: Testing /home/tester/tests/test-merge-1.sh
TESTLOG: testing level merge-1 branch macrochiropteran-jupon-lutecium
TESTLOG: Test /home/tester/tests/test-merge-1.sh passed
TESTLOG: Testing /home/tester/tests/test-merge-2.sh
TESTLOG: testing level merge-2 branch poseuse-citronwood-manganese
TESTLOG: Test /home/tester/tests/test-merge-2.sh passed
TESTLOG: Testing /home/tester/tests/test-merge-3.sh
TESTLOG: testing level merge-3 branch twee-enfamish-stropharia
TESTLOG: Test /home/tester/tests/test-merge-3.sh passed
TESTLOG: Testing /home/tester/tests/test-merge-4.sh
TESTLOG: testing level merge-4 branch multichord-ethicalism-fenestration
TESTLOG: Test /home/tester/tests/test-merge-4.sh passed
TESTLOG: Testing /home/tester/tests/test-merge-5.sh
TESTLOG: testing level merge-5 branch reappraise-veratroyl-garfishes
TESTLOG: Test /home/tester/tests/test-merge-5.sh passed
TESTLOG: Testing /home/tester/tests/test-revert-1.sh
TESTLOG: testing level revert-1 branch lomentaceous-mididae-hexadecane
TESTLOG: Test /home/tester/tests/test-revert-1.sh passed
TESTLOG: Testing /home/tester/tests/test-start-here.sh
TESTLOG: testing level start-here branch start-here
TESTLOG: Test /home/tester/tests/test-start-here.sh passed
TESTLOG: Out of 13 tests, 13 passed and 0 failed.
```

> Note: Can also run with -v to see all `git` output and random `echo`s as well.

## Develop

### Add a new stage

```powershell powershell
cargo run --bin generate-new-level -- ..\levels\game-config.toml .\src\bin\templates\level_checker.tmpl .\src\bin\templates\level_test.tmpl .\src\bin\templates\level_page.tmpl .\src\bin\resources\words_alpha.txt ..\levels\ -v
```
