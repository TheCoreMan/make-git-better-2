from ubuntu:latest

# Install dependencies.
RUN apt update -y
RUN DEBIAN_FRONTEND="noninteractive" apt install -y tzdata
RUN apt install -y \
	git-all \
	vim \
	nano \
	whois \
	openssh-server \
	curl \
	apt-utils \
	iputils-ping \
	zsh \
	tmux

# Create the required users. The game master is the `git` account, and the player is the user's account
# TODO - change the gamemaster password?
# TODO - change the gamemaster username to git?
RUN useradd --comment "GameMaster account" --create-home --password $(mkpasswd -m sha-512 gamemaster) gamemaster
RUN useradd --comment "Player account" --create-home --password $(mkpasswd -m sha-512 player) --shell /bin/zsh player

# Set up the player's SSH keys and copy the public key to /tmp
COPY build/player_entrypoint.sh /home/player
RUN ls -Rla /home/player
RUN chown player:player /home/player/player_entrypoint.sh
RUN chmod 770 /home/player/player_entrypoint.sh
RUN ls -Rla /home/player
RUN su -c "/home/player/player_entrypoint.sh" - player

RUN mkdir /var/run/sshd
RUN echo 'ClientAliveInterval 60' >> /etc/ssh/sshd_config
RUN echo 'ClientAliveCountMax 10' >>  /etc/ssh/sshd_config
COPY build/ssh_banner.txt /etc/banner
RUN echo 'Banner /etc/banner' >> /etc/ssh/sshd_config

# Set up the git server so that the player can run git clone gamemaster@localhost:/home/gamemaster/game-repo
RUN git clone --bare https://github.com/ShayNehmad/make-git-better-levels.git /home/gamemaster/game-repo
# This file adds the player's ssh public key from before
COPY build/gamemaster_entrypoint.sh /home/gamemaster
RUN chown gamemaster:gamemaster /home/gamemaster/gamemaster_entrypoint.sh
RUN chmod 770 /home/gamemaster/gamemaster_entrypoint.sh
RUN su -c "/home/gamemaster/gamemaster_entrypoint.sh" - gamemaster
# Set up the hooks for the actual gameplay in the repo
COPY levels/checkers /home/gamemaster/game-repo/hooks/checkers
COPY scripts/generate-pre-receive-hook/output/pre-receive /home/gamemaster/game-repo/hooks
# Make sure that gamemaster owns all of their files
RUN chown -R gamemaster:gamemaster /home/gamemaster

# Now that we're done with gamemaster's setup we can change his shell to git shell 
RUN chsh gamemaster -s $(which git-shell)

# Cleanup
RUN rm -rf /tmp/*
RUN rm -rf /home/player/player_entrypoint.sh

# Some debug messages
RUN ls -Rla /home

EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]

