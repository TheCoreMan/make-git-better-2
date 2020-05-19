from ubuntu:20.04

# Install dependencies.
RUN apt update -y
RUN apt install -y git vim nano whois

# Create the required users. The game master is the `git` account, and the player is the user's account
# TODO - change the gamemaster password?
# TODO - change the gamemaster username to git?
RUN useradd --comment "GameMaster account" --create-home --password $(mkpasswd -m sha-512 gamemaster) gamemaster
RUN useradd --comment "Player account" --create-home --password $(mkpasswd -m player) player

# Set up the player's SSH keys and copy the public key to /tmp
RUN su -c "ssh-keygen -q -t rsa -N '' -f ~/.ssh/id_rsa.player 2>/dev/null <<< y >/dev/null" - player
RUN cp /home/player/.ssh/id_rsa.player.pub /tmp

# Set up the git server so that the player can run git clone gamemaster@localhost:/home/gamemaster/repo 
RUN git clone --bare https://github.com/ShayNehmad/make-git-better-levels.git /home/gamemaster/repo
# This file adds the player's ssh public key from before
COPY gamemaster_entrypoint.sh /home/gamemaster
RUN chown gamemaster:gamemaster /home/gamemaster/gamemaster_entrypoint.sh
RUN chmod 770 /home/gamemaster/gamemaster_entrypoint.sh
RUN su -c "/home/gamemaster/gamemaster_entrypoint.sh" - gamemaster

# Set up the hooks for the actual gameplay in the repo
COPY levels/checkers /home/gamemaster/repo/hooks
COPY scripts/generate-pre-receive-hook/output/pre-receive /home/gamemaster/repo/hooks

# Now that we're done with gamemaster's setup we can change his shell to git shell 
RUN chsh gamemaster -s $(which git-shell)

# Cleanup
RUN rm -rf /tmp/*

