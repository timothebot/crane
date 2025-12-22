use std::fmt::Display;

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Shell::Bash =>
                    r#"
# START generated crane shell config
_crane() {
    local cur
    cur="${COMP_WORDS[COMP_CWORD]}"
    local subcommands="list add completions"
    if [[ $COMP_CWORD -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "$subcommands" -- "$cur") )
        return
    fi
    if [[ ${COMP_WORDS[1]} == add ]]; then
        local items
        items="$(crane list --no-format 2>/dev/null)"
        COMPREPLY=( $(compgen -W "$items" -- "$cur") )
        return
    fi
}
complete -F _crane crane
# END generated crane shell config"#,
                Shell::Zsh =>
                    r#"
# START generated crane shell config
_crane() {
  local -a subcmds add_args
  subcmds=(
    'add:Add bricks'
    'list:List bricks'
    'completions:Generate shell completions'
  )
  if (( CURRENT == 2 )); then
    _describe 'subcommand' subcmds
    return
  fi
  case $words[2] in
    add)
      _arguments "*:argument:($(crane list --no-format))"
      ;;
  esac
}
compdef _crane crane
# END generated crane shell config"#,
                Shell::Fish =>
                    r#"
# START generated crane shell config
complete -c crane -f -n '__fish_use_subcommand' -a list -d 'List bricks'
complete -c crane -f -n '__fish_use_subcommand' -a add -d 'Add bricks'
complete -c crane -f -n '__fish_use_subcommand' -a completions -d 'Generate shell completions'
complete -c crane -f -n '__fish_seen_subcommand_from add' \
    -a '(crane list --no-format 2>/dev/null)'
# END generated crane shell config"#,
            }
        )
    }
}
