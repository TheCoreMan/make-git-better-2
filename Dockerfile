from ubuntu:20.04

RUN apt update -y
RUN apt install -y git openssl vim nano

RUN useradd --comment "GameMaster account" --create-home --password $(openssl passwd -6 gamemaster) gamemaster
RUN useradd --comment "Player account" --create-home --password $(openssl passwd -6 player) player

RUN git clone https://github.com/ShayNehmad/make-git-better-levels.git /home/gamemaster/repo
COPY levels/checkers /home/gamemaster/repo/hooks
COPY scripts/generate-pre-receive-hook/output/pre-receive /home/gamemaster/repo/hooks

