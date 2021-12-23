# To initialize tutelnav put this in your bashrc
# eval "$(tutel init bash)" 

function tutelnav() {
  out=$(tutel nav query $1)
  if [ $? != 0 ]; then 
    return $?
  fi
  builtin cd $out
}

alias tnav="tutelnav"
alias tquery="tutel nav query"
alias tq="tutelnav query"
