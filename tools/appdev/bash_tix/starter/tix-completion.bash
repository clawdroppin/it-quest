# Tab completion for tix. Load it with:  source tix-completion.bash
# ("make install" copies it to where bash-completion finds it automatically.)
_tix() {
    local cur=${COMP_WORDS[COMP_CWORD]} prev=${COMP_WORDS[COMP_CWORD-1]}
    if (( COMP_CWORD == 1 )); then
        COMPREPLY=($(compgen -W "add list show assign close reopen report help" -- "$cur"))
        return
    fi
    case $prev in
        -p) COMPREPLY=($(compgen -W "urgent high normal low" -- "$cur")); return ;;
    esac
    case ${COMP_WORDS[1]} in
        show | assign | close | reopen)
            if (( COMP_CWORD == 2 )); then
                # offer the ticket numbers that exist
                local file=${TIX_FILE:-${XDG_DATA_HOME:-$HOME/.local/share}/tix/tickets.tsv}
                [[ -f $file ]] && COMPREPLY=($(compgen -W "$(cut -f1 "$file")" -- "$cur"))
            fi ;;
        add) COMPREPLY=($(compgen -W "-p -a" -- "$cur")) ;;
        list) COMPREPLY=($(compgen -W "-p -a -c" -- "$cur")) ;;
    esac
}
complete -F _tix tix
