use builtin;
use str;

set edit:completion:arg-completer[tutel] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'tutel'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'tutel'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand new 'create a new project'
            cand add 'add a new task'
            cand done 'mark a task as being completed'
            cand rm 'remove a task'
            cand edit 'edit an existing task'
            cand completions 'print shell completions'
        }
        &'tutel;new'= {
            cand -f 'force project creation'
            cand --force 'force project creation'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'tutel;add'= {
            cand -c 'mark the task as already completed'
            cand --completed 'mark the task as already completed'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'tutel;done'= {
            cand -! 'mark the task as not being done'
            cand --not 'mark the task as not being done'
            cand -a 'select all tasks'
            cand --all 'select all tasks'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'tutel;rm'= {
            cand -a 'remove all tasks'
            cand --all 'remove all tasks'
            cand -c 'remove all completed tasks'
            cand --cleanup 'remove all completed tasks'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'tutel;edit'= {
            cand -e 'the editor to use'
            cand --editor 'the editor to use'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'tutel;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
    ]
    $completions[$command]
}

