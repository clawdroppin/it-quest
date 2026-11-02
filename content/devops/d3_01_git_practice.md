@@lesson d3-git-practice Git and GitHub in Practice: Branches, Conflicts, Rebase and Pull Requests
@@concepts files, processes
@@lang bash
@@text
The Git lesson in module 0 explained the model. This one is hands-on: every exercise runs **real Git** in a fresh, empty folder, so you practise the exact commands you'll type at work. (On Windows the app uses Git Bash, which comes with Git.)

## Setting up a repository

```bash
cd "$(mktemp -d)"                          # a throwaway folder
git init -q -b main                         # -b main: name the first branch "main"
git config user.name "Aiko Tanaka"          # who you are (per repo; --global for every repo)
git config user.email "aiko@example.com"
echo "# Inventory scripts" > README.md
git add README.md
git commit -q -m "Add README"
git log --oneline | sed 's/^[0-9a-f]* //'   # the commit messages (hashes differ every time)
git status --short                          # nothing to show: the working tree is clean
```

The three places your changes live: the **working tree** (your files), the **staging area** or index (what the next commit will contain, filled with `git add`), and the **repository** (commits). `git status` shows which is which, `git diff` shows unstaged changes, and `git diff --staged` shows what's about to be committed.

## Branches: cheap, fast and everywhere

A branch is just a movable label pointing at a commit. Make one for every change:

```bash
cd "$(mktemp -d)" && git init -q -b main && git config user.name A && git config user.email a@x
echo "v1" > app.txt && git add . && git commit -q -m "First version"

git switch -c feature/login                 # create a branch and move to it (old: git checkout -b)
echo "login" >> app.txt
git commit -q -am "Add login"               # -a: stage every tracked, changed file
git switch main                             # back to main: app.txt is "v1" again
cat app.txt
git merge -q --no-ff -m "Merge feature/login" feature/login
git log --format=%s --graph
```

- A **fast-forward** merge just moves the label when main hasn't changed; `--no-ff` always creates a merge commit, so the history shows that a feature happened.
- Delete merged branches: `git branch -d feature/login`. List them: `git branch -a` (with remotes), `git branch --merged`.

## Merge conflicts

When both branches change the **same lines**, Git can't choose, so it stops and marks the file:

```text
<<<<<<< HEAD
timeout = 30
=======
timeout = 60
>>>>>>> feature/slow-network
```

To resolve: edit the file to what it *should* be (often a mix of both), delete the three marker lines, then `git add file` and `git commit` (or `git merge --continue`). Changed your mind? `git merge --abort` puts everything back. VS Code shows "Accept current / incoming / both" buttons above each conflict.

Avoid conflicts by keeping branches short-lived (a day or two), pulling `main` often, and not reformatting whole files in a feature branch.

## Rebase: a straight-line history

`git rebase main` (on your feature branch) replays your commits on top of the newest `main`, as if you had started from there. The history becomes a straight line, which is easier to read.

```text
before:  main  A──B──C            after:  main  A──B──C
                \                                     \
         feature D──E                       feature    D'──E'
```

**The golden rule: never rebase commits that others already pulled** (shared branches like `main`). Rebasing rewrites commits (D becomes D'), so anyone who had D now has a different history. Rebasing *your own* feature branch before opening a pull request is fine, and common.

Other history tools:

| Command | Does |
|---|---|
| `git commit --amend` | fix the last commit (message or content) |
| `git rebase -i HEAD~3` | interactively squash, reorder, reword or drop the last 3 commits |
| `git reset --soft HEAD~3` | un-commit the last 3 commits but keep their changes staged (a quick squash) |
| `git revert <commit>` | a **new** commit that undoes an old one: the safe undo for shared branches |
| `git restore file` | throw away unstaged changes to a file |
| `git stash` / `git stash pop` | park unfinished work, switch branches, bring it back |
| `git cherry-pick <commit>` | copy one commit onto the current branch (e.g. a hotfix) |
| `git reflog` | every place HEAD has been: how you recover "lost" commits |
| `git bisect` | binary-search the history for the commit that broke something |

## Remotes and GitHub

```text
git clone https://github.com/acme/infra.git     # copy a repository, with remote "origin"
git remote -v                                    # where origin points
git fetch                                        # download new commits, change nothing locally
git pull --rebase                                # fetch + rebase your local commits on top
git push -u origin feature/login                 # publish a branch; -u remembers the link
git push --force-with-lease                      # after rebasing YOUR branch: safe force push
```

Use SSH keys (`ssh-keygen -t ed25519`, add the public key in GitHub → Settings → SSH keys) or the GitHub CLI (`gh auth login`) instead of typing passwords. Never commit secrets: once pushed, a secret must be **rotated**, because deleting it doesn't remove it from history.

## The pull request workflow

1. `git switch -c fix/disk-alert-threshold` from an up-to-date `main`.
2. Small commits with clear messages: *imperative mood, under ~70 characters*: "Raise disk alert threshold to 90%", not "fixed stuff".
3. `git push -u origin fix/disk-alert-threshold`, then open a PR: `gh pr create --fill` or on github.com.
4. **CI runs** on the PR (tests, lint, `terraform plan`). Reviewers comment; you push more commits to the same branch.
5. When approved and green: **merge** (merge commit, squash, or rebase-merge, as the team prefers), and delete the branch.

Protect `main` in the repository settings (**branch protection / rulesets**): require a PR, at least one approving review, passing status checks, and no force pushes. Add a `CODEOWNERS` file so the right team is asked to review the files they own.

Workflows: **GitHub flow** (main + short feature branches; most teams), **trunk-based development** (tiny branches merged several times a day, unfinished features hidden behind feature flags), **Git flow** (develop/release/hotfix branches; for software with versioned releases).

## Useful files

- `.gitignore`: files Git should never track: `*.log`, `.env`, `__pycache__/`, `node_modules/`, `.terraform/`, `*.tfstate`.
- `.gitattributes`: line endings (`* text=auto eol=lf`) so Windows and Linux colleagues don't fight over CRLF.
- **Conventional Commits** (`feat:`, `fix:`, `docs:`, `chore:`) let tools build changelogs and version numbers automatically.
- **Tags** mark releases: `git tag -a v1.4.0 -m "Release 1.4.0"` and `git push --tags`.

@@quiz
You rebased your feature branch, and `git push` is rejected. What should you run?
- `git push --force` to overwrite whatever is there
* `git push --force-with-lease`, which only overwrites if nobody else pushed in the meantime
- `git pull` and merge, then push
- Delete the remote branch with the GitHub website
? Rebasing rewrote your commits, so the remote has the old ones. `--force-with-lease` is the safe way to replace them.

@@quiz
A bad commit is already on the shared `main` branch. What's the safe way to undo it?
- `git reset --hard HEAD~1` and force-push
* `git revert <commit>`, which adds a new commit that undoes it
- Delete the repository and push again
- `git commit --amend`
? Revert doesn't rewrite history, so nobody else's copy breaks.

@@quiz
What does `git merge --no-ff feature` do differently from a fast-forward merge?
- It refuses to merge if there are conflicts
* It always creates a merge commit, so the history shows the feature as a unit
- It merges without running hooks
- It deletes the feature branch
? A fast-forward only moves the `main` label forward when that's possible.

@@quiz
You accidentally committed an API key and pushed it. What must you do?
- Delete the file in a new commit; that's enough
- Make the repository private
* Rotate (revoke and replace) the key right away, then clean it from history if needed
- Run `git revert`
? The key is in the history and may already have been copied. Only rotating it makes it useless.

@@quiz
Which file makes GitHub ask a specific team to review changes to their files?
- `.gitignore`
- `README.md`
* `CODEOWNERS`
- `.gitattributes`
? Combined with branch protection, code owners' approval can be required before merging.

@@flashcards
working tree / staging area / repository :: your files / what the next commit holds / the commits
`git switch -c name` :: create a branch and move to it
`git commit -am "msg"` :: stage every changed tracked file and commit
`git merge --no-ff` :: always make a merge commit
conflict markers :: `<<<<<<<`, `=======`, `>>>>>>>`: edit, delete them, add, commit
`git merge --abort` :: give up on a merge and go back
`git rebase main` :: replay my commits on top of the newest main
golden rule of rebase :: never rebase commits others already have
`git rebase -i HEAD~3` :: squash, reorder, reword or drop recent commits
`git reset --soft HEAD~3` :: un-commit 3 commits, keep the changes staged
`git revert <c>` :: a new commit that undoes `<c>`: safe on shared branches
`git stash` / `git stash pop` :: park and restore unfinished work
`git cherry-pick <c>` :: copy one commit onto this branch
`git reflog` :: recover "lost" commits
`git bisect` :: find the commit that broke something
`git push --force-with-lease` :: force push that won't clobber others' work
pull request :: propose merging a branch; review and CI happen here
branch protection :: require PRs, reviews and green checks on main
`CODEOWNERS` :: who must review which files
Conventional Commits :: `feat:`, `fix:` prefixes for automatic changelogs
`git tag -a v1.0.0` :: mark a release

@@exercise A feature branch and a merge
The starter creates a repository with one commit. Create a branch `feature/backup`, add a line `backup=daily` to `config.txt` and commit it with the message `Enable daily backups`. Switch back to `main` and merge the branch **with a merge commit** (`--no-ff`) using the message `Merge feature/backup`. The last lines print main's own history (`--first-parent`), the commit that was merged in (`HEAD^2` is the merge's second parent), and the file.
@@starter
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
echo "name=fileserver" > config.txt
git add config.txt
git commit -q -m "Add config"

# Your commands here


git log --first-parent --format=%s
echo "merged in: $(git log -1 --format=%s HEAD^2)"
cat config.txt
@@expected
Merge feature/backup
Add config
merged in: Enable daily backups
name=fileserver
backup=daily
@@requires
git switch
feature/backup
--no-ff
@@hint
`git switch -c feature/backup`, then `echo "backup=daily" >> config.txt`, `git commit -q -am "Enable daily backups"`, `git switch -q main`, `git merge -q --no-ff -m "Merge feature/backup" feature/backup`.
@@solution
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
echo "name=fileserver" > config.txt
git add config.txt
git commit -q -m "Add config"

git switch -q -c feature/backup
echo "backup=daily" >> config.txt
git commit -q -am "Enable daily backups"
git switch -q main
git merge -q --no-ff -m "Merge feature/backup" feature/backup

git log --first-parent --format=%s
echo "merged in: $(git log -1 --format=%s HEAD^2)"
cat config.txt

@@exercise Resolve a merge conflict
The starter sets up a conflict: `main` changed the timeout to `45` and the branch `feature/retry` changed it to `90` and added `retries=3`. The merge stops with a conflict. Resolve it so `settings.txt` ends up as exactly the two lines `timeout=90` and `retries=3`, then finish the merge with the message `Merge feature/retry`. No conflict markers may remain.
@@starter
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
echo "timeout=30" > settings.txt
git add . && git commit -q -m "Add settings"
git switch -q -c feature/retry
printf 'timeout=90\nretries=3\n' > settings.txt
git commit -q -am "Slower network: longer timeout and retries"
git switch -q main
echo "timeout=45" > settings.txt
git commit -q -am "Timeout 45"
git merge -q feature/retry > /dev/null 2>&1 || echo "conflict!"

# Resolve it here


echo "---"
cat settings.txt
git log -1 --format=%s
git status --short | wc -l
@@expected
conflict!
---
timeout=90
retries=3
Merge feature/retry
0
@@requires
git add settings.txt
@@hint
Overwrite the file with the resolved content: `printf 'timeout=90\nretries=3\n' > settings.txt`, then `git add settings.txt` and `git commit -q -m "Merge feature/retry"`.
@@solution
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
echo "timeout=30" > settings.txt
git add . && git commit -q -m "Add settings"
git switch -q -c feature/retry
printf 'timeout=90\nretries=3\n' > settings.txt
git commit -q -am "Slower network: longer timeout and retries"
git switch -q main
echo "timeout=45" > settings.txt
git commit -q -am "Timeout 45"
git merge -q feature/retry > /dev/null 2>&1 || echo "conflict!"

printf 'timeout=90\nretries=3\n' > settings.txt
git add settings.txt
git commit -q -m "Merge feature/retry"

echo "---"
cat settings.txt
git log -1 --format=%s
git status --short | wc -l

@@exercise Squash before a pull request
Your branch has three messy commits. Before opening a pull request, squash them into **one** commit with the message `Add disk report script`, using `git reset --soft`. Keep all the changes.
@@starter
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
echo "# tools" > README.md && git add . && git commit -q -m "Initial commit"
git switch -q -c feature/disk-report
echo 'df -h' > report.sh && git add . && git commit -q -m "wip"
echo 'du -sh /var/log' >> report.sh && git commit -q -am "more wip"
echo 'echo done' >> report.sh && git commit -q -am "fix typo"

# Squash the last three commits into one here


git log --format=%s
cat report.sh
@@expected
Add disk report script
Initial commit
df -h
du -sh /var/log
echo done
@@requires
git reset --soft
@@hint
`git reset --soft HEAD~3` un-commits the three commits but keeps their changes staged. Then `git commit -q -m "Add disk report script"`.
@@solution
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
echo "# tools" > README.md && git add . && git commit -q -m "Initial commit"
git switch -q -c feature/disk-report
echo 'df -h' > report.sh && git add . && git commit -q -m "wip"
echo 'du -sh /var/log' >> report.sh && git commit -q -am "more wip"
echo 'echo done' >> report.sh && git commit -q -am "fix typo"

git reset --soft HEAD~3
git commit -q -m "Add disk report script"

git log --format=%s
cat report.sh

@@exercise Undo safely with revert
A commit that turned off the firewall was merged into `main`, which others have already pulled. Undo it **without rewriting history**: use `git revert` on that commit (find it by its message with `git log --grep`), keeping Git's default message (`--no-edit`).
@@starter
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
printf 'firewall=on\nlog=info\n' > host.conf && git add . && git commit -q -m "Add host config"
sed -i 's/firewall=on/firewall=off/' host.conf && git commit -q -am "Disable firewall for testing"
echo "ntp=pool.ntp.org" >> host.conf && git commit -q -am "Add NTP server"

# Undo the "Disable firewall" commit here


git log --format=%s
cat host.conf
@@expected
Revert "Disable firewall for testing"
Add NTP server
Disable firewall for testing
Add host config
firewall=on
log=info
ntp=pool.ntp.org
@@requires
git revert
@@hint
`bad=$(git log --grep="Disable firewall" --format=%H)` finds the commit's hash; then `git revert --no-edit "$bad"`. Add `-q` or `> /dev/null` to hide Git's summary line.
@@solution
cd "$(mktemp -d)"
git init -q -b main
git config user.name "Learner"
git config user.email "learner@example.com"
git config core.autocrlf false
printf 'firewall=on\nlog=info\n' > host.conf && git add . && git commit -q -m "Add host config"
sed -i 's/firewall=on/firewall=off/' host.conf && git commit -q -am "Disable firewall for testing"
echo "ntp=pool.ntp.org" >> host.conf && git commit -q -am "Add NTP server"

bad=$(git log --grep="Disable firewall" --format=%H)
git revert --no-edit "$bad" > /dev/null

git log --format=%s
cat host.conf
