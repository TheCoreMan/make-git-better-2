FROM ubuntu:focal

# Users will log into this machine, so we need to unminimize it.
# See https://wiki.ubuntu.com/Minimal
RUN yes | unminimize

# Install dependencies.
RUN apt update -y
RUN apt clean -y
RUN DEBIAN_FRONTEND="noninteractive" apt install -y tzdata
RUN apt clean -y
RUN apt update -y && apt install -y \
    git-all \
    vim \
    nano \
    whois \
    openssh-server \
    curl \
    apt-utils \
    iputils-ping \
    zsh \
    tmux \
    man \
    fzf \ 
    sudo \
    jq

# Create the required users. The game master is the `git` account, and the player is the user's account
RUN useradd --comment "GameMaster account" --create-home --password $(mkpasswd -m sha-512 95+mcguffin+STRONG+ainasdf+15) gamemaster
RUN useradd --comment "Player account" --create-home --password $(mkpasswd -m sha-512 player) --shell $(which zsh) player
RUN useradd --comment "Testing account" --create-home --password $(mkpasswd -m sha-512 tester) --shell $(which zsh) tester

# OWASP addition
RUN useradd --comment "1st Flag account" --create-home --password $(mkpasswd -m sha-512 flagger) --shell $(which bash) flagger
RUN useradd --comment "2nd Flag account" --create-home --password $(mkpasswd -m sha-512 flagger_the_second) --shell $(which bash) flagger_the_second
RUN useradd --comment "Build system account" --create-home --password $(mkpasswd -m sha-512 build_system) --shell $(which bash) build_system
RUN mkdir -p /etc/owasp/flags
ARG OWASP_FLAG_1
RUN echo $OWASP_FLAG_1 > /etc/owasp/flags/flag.txt
ARG OWASP_FLAG_2
RUN echo $OWASP_FLAG_2 > /flag.txt
RUN chown --verbose --recursive flagger /etc/owasp
RUN chmod --verbose --recursive 0700 /etc/owasp
RUN chown --verbose flagger_the_second /flag.txt
RUN chmod --verbose 0400 /flag.txt
# OWASP additions
RUN echo "gamemaster     ALL=(flagger) NOPASSWD:/usr/bin/whoami, /usr/bin/python3" >> /etc/sudoers
RUN echo "gamemaster     ALL=(flagger_the_second) NOPASSWD:/usr/bin/whoami, /usr/bin/cp, /usr/bin/chown" >> /etc/sudoers
RUN echo "gamemaster     ALL=(build_system) NOPASSWD: ALL" >> /etc/sudoers
RUN echo "gamemaster     ALL=(ALL) NOPASSWD:/usr/bin/chown" >> /etc/sudoers

# Set up the player's SSH keys and copy the public key to /tmp
COPY build/player_entrypoint.sh /home/player
COPY build/keys/id_rsa.player /home/player/.ssh/id_rsa
COPY build/keys/id_rsa.player.pub /home/player/.ssh/id_rsa.pub
RUN chown player:player /home/player/player_entrypoint.sh
RUN chmod 770 /home/player/player_entrypoint.sh
RUN su -c "/home/player/player_entrypoint.sh" - player
COPY build/player_zshrc.sh /home/player/.zshrc
RUN chown player:player /home/player/.zshrc
RUN chmod 770 /home/player/.zshrc

# Do the same for the tester account.
COPY build/tester_entrypoint.sh /home/tester
RUN chown tester:tester /home/tester/tester_entrypoint.sh
RUN chmod 770 /home/tester/tester_entrypoint.sh
RUN su -c "/home/tester/tester_entrypoint.sh" - tester
COPY build/tester_zshrc.sh /home/tester/.zshrc
RUN chown tester:tester /home/tester/.zshrc
RUN chmod 770 /home/tester/.zshrc
# Copy the test files to the tester account
COPY levels/tests /home/tester/tests
RUN chown --recursive tester:tester /home/tester

# Set up SSH
RUN mkdir /var/run/sshd
COPY build/sshd_config /etc/ssh/sshd_config
COPY build/login_banner.txt /etc/motd

RUN /etc/init.d/ssh start && ssh-keyscan -H localhost >> /home/player/.ssh/known_hosts && ssh-keyscan -H localhost >> /home/tester/.ssh/known_hosts

# Set up the git server so that the player can run git clone gamemaster@localhost:/home/gamemaster/ctf-repo
RUN git clone --bare https://github.com/TheCoreMan/make-git-better-levels.git /home/gamemaster/ctf-repo
# Set up the other remote for the remote stages
RUN git clone --bare https://github.com/sandspider2234/make-git-better-levels.git /home/gamemaster/forked-ctf-repo
# This file adds the player's ssh public key from before
COPY build/gamemaster_entrypoint.sh /home/gamemaster
RUN chown gamemaster:gamemaster /home/gamemaster/gamemaster_entrypoint.sh
RUN chmod 770 /home/gamemaster/gamemaster_entrypoint.sh
# Make sure that gamemaster owns all of their files
RUN chown --recursive gamemaster:gamemaster /home/gamemaster
# This arg invalidates cache from here on forward. use the current time (no spaces) as a build arg.
ARG CACHE_DATE
RUN echo "This CTF server was built at "$CACHE_DATE"." >> /etc/motd
RUN ls -la "/home/gamemaster"
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
RUN rm -rf /root/.ssh/

EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]
