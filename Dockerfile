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
RUN useradd --comment "GameMaster account" --create-home --password $(mkpasswd -m sha-512 94+wings+STRONG+mountain+35) gamemaster
RUN useradd --comment "Player account" --create-home --password $(mkpasswd -m sha-512 player) --shell $(which zsh) player

# Set up the player's SSH keys and copy the public key to /tmp
COPY build/player_entrypoint.sh /home/player
RUN chown player:player /home/player/player_entrypoint.sh
RUN chmod 770 /home/player/player_entrypoint.sh
RUN su -c "/home/player/player_entrypoint.sh" - player
COPY build/player_zshrc.sh /home/player/.zshrc
RUN chown player:player /home/player/.zshrc
RUN chmod 770 /home/player/.zshrc

RUN mkdir /var/run/sshd
RUN echo 'ClientAliveInterval 60' >> /etc/ssh/sshd_config
RUN echo 'ClientAliveCountMax 10' >>  /etc/ssh/sshd_config
RUN sed -ri 's/UsePAM yes/#UsePAM yes/g' /etc/ssh/sshd_config
RUN echo 'UsePAM no' >> /etc/ssh/sshd_config
RUN echo 'PrintMotd yes' >> /etc/ssh/sshd_config
COPY build/login_banner.txt /etc/motd

# Set up the git server so that the player can run git clone gamemaster@localhost:/home/gamemaster/ctf-repo
RUN git clone --bare https://github.com/ShayNehmad/make-git-better-levels.git /home/gamemaster/ctf-repo
# This file adds the player's ssh public key from before
COPY build/gamemaster_entrypoint.sh /home/gamemaster
RUN chown gamemaster:gamemaster /home/gamemaster/gamemaster_entrypoint.sh
RUN chmod 770 /home/gamemaster/gamemaster_entrypoint.sh
# Make sure that gamemaster owns all of their files
RUN chown -R gamemaster:gamemaster /home/gamemaster
# This arg invalidates cache from here on forward. use the current time (no spaces) as a build arg.
ARG CACHE_DATE=not_a_date
RUN su -c "/home/gamemaster/gamemaster_entrypoint.sh" - gamemaster
# Set up the hooks for the actual gameplay in the repo
COPY levels/checkers /home/gamemaster/ctf-repo/hooks/checkers
COPY scripts/output/pre-receive /home/gamemaster/ctf-repo/hooks
# Make sure that gamemaster owns all of their files
RUN chown -R gamemaster:gamemaster /home/gamemaster

# Now that we're done with gamemaster's setup we can change their shell to git shell and block their home directory
RUN chsh gamemaster -s $(which git-shell)
RUN chmod 700 -R /home/gamemaster

# Cleanup
RUN rm -rf /tmp/*
RUN rm -rf /home/player/player_entrypoint.sh

EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]

