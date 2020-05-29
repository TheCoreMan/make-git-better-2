#!/bin/bash

set -x

if [[ ! $(whoami) == "gamemaster" ]] 
	then echo "I'm not the gamemaster"; exit 1; 
fi

if [[ ! -f /tmp/id_rsa.player.pub ]]
	then echo "Not public key file found"; exit 1;
fi

# https://git-scm.com/book/en/v2/Git-on-the-Server-Setting-Up-the-Server
cd
pwd
mkdir .ssh && chmod 700 .ssh
cat /tmp/id_rsa.player.pub >> ~/.ssh/authorized_keys

cd ~/ctf-repo
echo In $(pwd), updating bare repo...
ls -la
git fetch origin +refs/heads/*:refs/heads/* --prune
