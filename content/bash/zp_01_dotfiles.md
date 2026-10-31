@@lesson xp-dotfiles Dotfiles Installer
@@text
Your aliases, prompt, Git config and editor settings, in one Git repository, with an `install.sh` that symlinks them into your home folder, backs up anything it would replace, and can be run again safely.

**You'll practise:** symlinks (ln -s) · idempotent scripts · backups with timestamps · set -euo pipefail

## Milestones

1. **Collect**: put .bashrc, .gitconfig and .vimrc (or .inputrc) in the dotfiles folder.
2. **Link**: for each file, create `~/<file>` as a symlink to the repo copy.
3. **Back up**: if a real file is there already, move it to `~/.dotfiles-backup/<date>/`.
4. **Idempotent**: running it twice changes nothing the second time and says so.
5. **Uninstall**: `install.sh --uninstall` removes the links and restores the backups.

## You're done when

- On a fresh VM or WSL, `git clone` plus `./install.sh` gives you your setup.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"How do I check whether a path is already a symlink to the right place?"*
- *"Review my script for quoting bugs with filenames that contain spaces."*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project dotfiles Dotfiles Installer
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# Dotfiles Installer

Your aliases, prompt, Git config and editor settings, in one Git repository, with an `install.sh` that symlinks them into your home folder, backs up anything it would replace, and can be run again safely.

## Milestones

- [ ] 1. Collect: put .bashrc, .gitconfig and .vimrc (or .inputrc) in the dotfiles folder.
- [ ] 2. Link: for each file, create `~/<file>` as a symlink to the repo copy.
- [ ] 3. Back up: if a real file is there already, move it to `~/.dotfiles-backup/<date>/`.
- [ ] 4. Idempotent: running it twice changes nothing the second time and says so.
- [ ] 5. Uninstall: `install.sh --uninstall` removes the links and restores the backups.

## Done when

- [ ] On a fresh VM or WSL, `git clone` plus `./install.sh` gives you your setup.
@@file install.sh
#!/usr/bin/env bash
# Symlink every file in ./home into $HOME, backing up anything that's in the way.
set -euo pipefail

repo="$(cd "$(dirname "$0")" && pwd)"
backup="$HOME/.dotfiles-backup/$(date +%Y%m%d-%H%M%S)"

for src in "$repo"/home/.[!.]*; do
  name="$(basename "$src")"
  dest="$HOME/$name"
  # TODO: skip if $dest already links to $src
  # TODO: back up a real file at $dest into $backup
  # TODO: ln -s "$src" "$dest"
  echo "would link $dest -> $src"
done
@@file home/.bash_aliases
alias ll='ls -lah'
alias gs='git status -sb'
alias ..='cd ..'
