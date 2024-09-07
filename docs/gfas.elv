
use builtin;
use str;

set edit:completion:arg-completer[gfas] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'gfas'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'gfas'= {
            cand --endpoint 'GitHub API endpoint'
            cand --no-color 'Disable color printing'
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
