# To initialize tutelnav put this in your fish.conf
# tutel init fish | source

function tutelnav
  set -l out (tutel nav query $argv)
  if [ $status != 0 ]
    return $status
  end
  builtin cd $out
end

alias tnav=tutelnav
alias tquery="tutel nav query"
alias tq=tquery

