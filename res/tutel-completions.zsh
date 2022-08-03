#compdef tutel

autoload -U is-at-least

_tutel() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_tutel_commands" \
"*::: :->tutel" \
&& ret=0
    case $state in
    (tutel)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:tutel-command-$line[1]:"
        case $line[1] in
            (new)
                _arguments "${_arguments_options[@]}" \
                '-f[force project creation]' \
                '--force[force project creation]' \
                '-h[Prints help information]' \
                '--help[Prints help information]' \
                '::name -- the project name:' \
                && ret=0
            ;;
            (add)
                _arguments "${_arguments_options[@]}" \
                '-c[mark the task as already completed]' \
                '--completed[mark the task as already completed]' \
                '-h[Prints help information]' \
                '--help[Prints help information]' \
                '*::description -- the description of the task to add:' \
                && ret=0
            ;;
            (done)
                _arguments "${_arguments_options[@]}" \
                '-![mark the task as not being done]' \
                '--not[mark the task as not being done]' \
                '-a[select all tasks]' \
                '--all[select all tasks]' \
                '-h[Prints help information]' \
                '--help[Prints help information]' \
                '*::indices -- the task(s) index/indices:' \
                && ret=0
            ;;
            (rm)
                _arguments "${_arguments_options[@]}" \
                '(-c --cleanup)-a[remove all tasks]' \
                '(-c --cleanup)--all[remove all tasks]' \
                '(-a --all)-c[remove all completed tasks]' \
                '(-a --all)--cleanup[remove all completed tasks]' \
                '-h[Prints help information]' \
                '--help[Prints help information]' \
                '*::indices -- the task(s) index/indices:' \
                && ret=0
            ;;
            (edit)
                _arguments "${_arguments_options[@]}" \
                '-e+[the editor to use]: : ' \
                '--editor=[the editor to use]: : ' \
                '-h[Prints help information]' \
                '--help[Prints help information]' \
                ':index -- the task index:' \
                && ret=0
            ;;
            (completions)
                _arguments "${_arguments_options[@]}" \
                '-h[Prints help information]' \
                '--help[Prints help information]' \
                ':shell:(bash zsh fish elvish)' \
                && ret=0
            ;;
        esac
    ;;
esac
}

(( $+functions[_tutel_commands] )) ||
_tutel_commands() {
    local commands; commands=(
'new:create a new project' \
'add:add a new task' \
'done:mark a task as being completed' \
'rm:remove a task' \
'edit:edit an existing task' \
'completions:print shell completions' \
    )
    _describe -t commands 'tutel commands' commands "$@"
}
(( $+functions[_tutel__add_commands] )) ||
_tutel__add_commands() {
    local commands; commands=()
    _describe -t commands 'tutel add commands' commands "$@"
}
(( $+functions[_tutel__completions_commands] )) ||
_tutel__completions_commands() {
    local commands; commands=()
    _describe -t commands 'tutel completions commands' commands "$@"
}
(( $+functions[_tutel__done_commands] )) ||
_tutel__done_commands() {
    local commands; commands=()
    _describe -t commands 'tutel done commands' commands "$@"
}
(( $+functions[_tutel__edit_commands] )) ||
_tutel__edit_commands() {
    local commands; commands=()
    _describe -t commands 'tutel edit commands' commands "$@"
}
(( $+functions[_tutel__new_commands] )) ||
_tutel__new_commands() {
    local commands; commands=()
    _describe -t commands 'tutel new commands' commands "$@"
}
(( $+functions[_tutel__rm_commands] )) ||
_tutel__rm_commands() {
    local commands; commands=()
    _describe -t commands 'tutel rm commands' commands "$@"
}

_tutel "$@"

