@@lesson d0-git Git and Collaboration
@@text
**DevOps** is a culture and a set of practices that bring development and operations together: small changes, shipped often, automated, measured, and owned end to end ("you build it, you run it"). Everything starts with **version control**, and that means **Git**.

## The model

```text
working tree ──git add──▶ staging area (index) ──git commit──▶ local repository ──git push──▶ remote (GitHub)
                                                                      ◀──git pull / fetch──
```

A **commit** is a snapshot with an author, a message and a parent. A **branch** is just a movable pointer to a commit. **HEAD** is where you are now.

```bash,norun
git config --global user.name "Yuki Tanaka"
git config --global user.email "yuki@example.com"
git clone https://github.com/org/repo.git
git status                       # what changed? (run it constantly)
git diff                         # unstaged changes ; git diff --staged: staged ones
git add file.py ; git add -p     # stage a file ; stage pieces interactively
git commit -m "Fix DNS timeout in health check"
git log --oneline --graph --all
git switch -c feature/vpn-docs   # new branch ; git switch main: go back
git push -u origin feature/vpn-docs
git pull                         # fetch + merge (or rebase) from the remote
git restore file.py              # throw away unstaged changes to a file
git restore --staged file.py     # unstage
git revert <commit>              # a NEW commit that undoes one: safe on shared branches
git stash ; git stash pop        # park unfinished work
```

## Branching and merging

- **Merge** joins branches with a merge commit (or a **fast-forward** if nothing diverged). History shows what really happened.
- **Rebase** replays your commits on top of another branch: a straight, tidy history. **Never rebase commits others already pulled**: rewriting shared history breaks their copies. The same goes for `git push --force` on shared branches (if you must, use `--force-with-lease`).
- A **merge conflict** happens when both sides changed the same lines. Git marks them:

```text
<<<<<<< HEAD
timeout = 5
=======
timeout = 10
>>>>>>> feature/longer-timeout
```

Edit the file to the right result, remove the markers, `git add` it and commit (or `git rebase --continue`).

## Working as a team

- **Pull requests** (GitHub/Bitbucket) or **merge requests** (GitLab): propose a branch, discuss, review, run CI, then merge. **Code review** catches bugs and spreads knowledge.
- **Trunk-based development**: short-lived branches merged to `main` daily, behind **feature flags** if unfinished. **GitFlow** (develop/release/hotfix branches) is heavier and fading.
- **Branch protection**: no direct pushes to `main`, required reviews and passing checks.
- Good commit messages: a short imperative summary ("Add retry to backup upload"), then why.
- **`.gitignore`** keeps build output, virtual environments and **secrets** out. If a secret is ever committed, **revoke it**: deleting it in a later commit leaves it in the history. Tools like **gitleaks** and GitHub **secret scanning** catch them.
- Tag releases: `git tag v1.4.0 && git push --tags`. **Semantic versioning**: MAJOR.MINOR.PATCH.

Infrastructure people use Git too: scripts, Ansible playbooks, Terraform, Kubernetes YAML, network configs, documentation. If it isn't in Git, it doesn't exist.

@@quiz
You changed three files but only want to commit one of them. What do you do?
* `git add` that one file, then `git commit`
- `git commit -a`
- `git push` the file
- `git stash` all three
? The staging area lets you choose exactly what goes into a commit.

@@quiz
A bad commit is already on the shared `main` branch. What's the safe way to undo it?
* `git revert <commit>`, which adds a new commit undoing it
- `git reset --hard` and force-push
- Delete the repository
- Rebase main
? Rewriting shared history breaks everyone else's copies.

@@quiz
When should you NOT rebase?
* On commits that others have already pulled (shared history)
- On your own local feature branch
- Before opening a pull request
- To update your branch with main
? Rebase rewrites commits; that's fine only while they're yours alone.

@@quiz
A password was committed and pushed, then deleted in the next commit. Is it safe?
* No: it's still in the history. Revoke and rotate it
- Yes, the latest version doesn't contain it
- Yes, if the repository is private
- Only if nobody cloned it
? Treat any pushed secret as compromised.

@@quiz
What is a pull request for?
* Proposing changes from a branch so they can be reviewed and tested before merging
- Downloading the latest code
- Deleting a branch
- Reverting a commit
? CI runs on it, reviewers comment, then it's merged.

@@quiz
Which file tells Git which files to never track?
* .gitignore
- .gitconfig
- .gitattributes
- README.md
? Add build output, virtual environments, logs and secret files.

@@quiz
In trunk-based development, how do you merge a feature that isn't finished yet?
* Behind a feature flag, so the code ships but stays switched off
- You can't; keep the branch open for months
- Comment the code out
- Merge it to a separate trunk
? Long-lived branches cause painful merges.

@@flashcards
git status :: What changed and what's staged
git add :: Stage changes for the next commit
git commit -m :: Save a snapshot with a message
git push / git pull :: Send to / get from the remote
git switch -c NAME :: Create and switch to a branch
git log --oneline --graph :: Compact history
git diff --staged :: Changes that will be committed
git restore FILE :: Discard unstaged changes
git revert :: New commit that undoes an old one
git stash :: Park uncommitted work
merge vs rebase :: Join with a merge commit vs replay on top
fast-forward :: Merge that just moves the pointer
merge conflict :: Both sides changed the same lines
pull request :: Review + CI before merging
branch protection :: Rules for main: reviews, checks, no force-push
trunk-based development :: Short-lived branches, merge daily
feature flag :: Ship code switched off
semantic versioning :: MAJOR.MINOR.PATCH
--force-with-lease :: Force-push that refuses if someone else pushed

@@drill
Which Git command creates a new commit that undoes an earlier commit? (git ...)
= git revert
? Unlike reset, it doesn't rewrite history.
