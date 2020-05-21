#!/bin/zsh

if [[ ! $(whoami) == "player" ]] 
	then echo "I'm not the player"; exit 1; 
fi

# https://git-scm.com/book/en/v2/Git-on-the-Server-Setting-Up-the-Server
cd
pwd
ssh-keygen -q -t rsa -N '' -f ~/.ssh/id_rsa 2>/dev/null <<< y >/dev/null

cat ~/.ssh/id_rsa.pub >> /tmp/id_rsa.player.pub

echo "Setting up zsh"
sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"


git config --global user.email "player@mrnice.dev"
git config --global user.name "CTF player"

