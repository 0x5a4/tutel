# To initialize tutelnav put this in your fish.conf
# tutel init fish | source

function __tutelnav
  set -l out (tutel query $argv)
  if [ $status != 0 ]
    return $status
  end
  builtin cd $out
end

alias tutelnav=__tutelnav
