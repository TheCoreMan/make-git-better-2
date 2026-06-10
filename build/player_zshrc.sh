export ZSH="/home/player/.oh-my-zsh"
ZSH_THEME="juanghurtado"
plugins=(git)

source $ZSH/oh-my-zsh.sh

source /usr/share/doc/fzf/examples/key-bindings.zsh
source /usr/share/doc/fzf/examples/completion.zsh

alias python=python3

# See https://twitter.com/_bsless/status/1274072996689436674
if [[ "$TERM" == "dumb" ]]
then
  unsetopt zle
  unsetopt prompt_cr
  unsetopt prompt_subst
  unfunction precmd
  unfunction preexec
  PS1='$ '
fi
