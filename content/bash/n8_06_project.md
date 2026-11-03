@@lesson n8-project Project: A Release Script
@@concepts automation, strings, processes
@@text
This project ties Production Bash together: you'll build `release.sh`, which turns a list of commits into a new version number and a changelog, with a dry-run mode and safety checks, the way real teams automate releases.

## Conventional Commits

Many projects write commit messages in a fixed format so tools can read them:

```text
feat(api): add pagination to /users
fix: handle empty config file
docs: update README
feat!: drop support for Bash 3      ← "!" or a "BREAKING CHANGE:" footer
```

From these, **semantic versioning** follows mechanically:

| Commits since the last tag contain | Bump | 1.4.2 → |
|---|---|---|
| a breaking change (`!` or `BREAKING CHANGE`) | major | 2.0.0 |
| a `feat` | minor | 1.5.0 |
| only `fix`, `perf`, `docs`, `chore`... | patch | 1.4.3 |

(While the major version is 0, many projects bump minor for breaking changes instead.)

## The real Git commands

```bash,norun
last=$(git describe --tags --abbrev=0)               # e.g. v1.4.2
git log --format='%h %s' "$last"..HEAD               # commits since then
git status --porcelain                               # empty = clean working tree
git rev-parse --abbrev-ref HEAD                      # current branch
git tag -a "v$next" -m "Release v$next"
git push origin "v$next"
```

## Safety checks before releasing

- The working tree is clean (no uncommitted changes would be left out).
- You're on the release branch.
- There are commits since the last tag.
- The new tag doesn't already exist.
- In **dry-run** mode (`-n`), print every command instead of running it. Build commands as arrays and pass them to a `run` function:

```bash,norun
run() {
    if (( dry_run )); then printf '[dry-run]'; printf ' %q' "$@"; echo
    else "$@"; fi
}
run git tag -a "v$next" -m "Release v$next"
```

## Changelog format

Group the commits by type, newest first within each group, and skip noise like `chore` and `ci`:

```markdown
## v1.5.0 (2026-11-03)

### ⚠ Breaking changes
- ...
### Features
- **api:** add pagination to /users (a1b2c3d)
### Bug fixes
- handle empty config file (e4f5a6b)
```

@@quiz
The commits since v1.4.2 are `fix: x` and `feat(ui): y`. What's the next version?
- 1.4.3
* 1.5.0
- 2.0.0
- 1.4.2-1
? The highest bump wins: feat → minor.

@@quiz
Why check `git status --porcelain` before tagging a release?
* Uncommitted changes wouldn't be in the tagged commit, so the release wouldn't match what you tested
- Tags fail on dirty trees
- It's faster
- To push faster
? Releases must be reproducible from the tag.

@@quiz
What's the benefit of a `run` wrapper that prints commands in dry-run mode?
- It hides errors
* You see exactly what would happen, with the same code path as a real run
- It runs commands twice
- It's required by git
? Build commands as arrays so printing and running use the same data.

@@flashcards
Conventional Commits :: `type(scope)!: subject`
`feat` :: minor bump
`fix` :: patch bump
`!` / BREAKING CHANGE :: major bump
`git describe --tags --abbrev=0` :: latest tag
`git log A..HEAD` :: commits since A
`git status --porcelain` :: machine-readable status
dry run :: print instead of execute

@@exercise Next version and changelog
Write:

- `parse_commit LINE`: splits `hash type(scope)!: subject` with one regex into the globals `c_hash c_type c_scope c_breaking c_subject`; returns 1 for lines that don't follow the format
- `next_version CURRENT BUMP`: bumps `major`, `minor` or `patch`
- a changelog with sections in the order Breaking changes, Features, Bug fixes, Performance (skip `docs`, `chore`, `ci`, `test`), scope in bold, short hash in parentheses
- report non-conforming commits at the end
@@starter
current=1.4.2
commits='a1b2c3d feat(api): add pagination to /users
e4f5a6b fix: handle empty config file
0f9e8d7 docs: update README
c0ffee1 perf(db): cache prepared statements
badc0de Merge branch main
7d6e5f4 feat(cli)!: rename --dir to --path
1a2b3c4 chore: bump dependencies
9f8e7d6 fix(api): return 404 for unknown users'
echo TODO
@@expected
bump: major (1.4.2 -> 2.0.0)

## v2.0.0 (2026-11-03)

### ⚠ Breaking changes
- **cli:** rename --dir to --path (7d6e5f4)

### Features
- **api:** add pagination to /users (a1b2c3d)
- **cli:** rename --dir to --path (7d6e5f4)

### Bug fixes
- handle empty config file (e4f5a6b)
- **api:** return 404 for unknown users (9f8e7d6)

### Performance
- **db:** cache prepared statements (c0ffee1)

skipped (not conventional): badc0de Merge branch main
@@requires
BASH_REMATCH
declare -A
@@hint
`re='^([0-9a-f]+) ([a-z]+)(\(([a-z0-9-]+)\))?(!)?: (.+)$'`: groups 1 hash, 2 type, 4 scope, 5 `!`, 6 subject. Collect entries per type in an associative array of newline-separated lines (`sections[$c_type]+="$entry"$'\n'`), plus a `breaking` string. The bump is the maximum seen: track it as a number (0 patch, 1 minor, 2 major). Use a fixed date so the output is stable.
@@solution
current=1.4.2
commits='a1b2c3d feat(api): add pagination to /users
e4f5a6b fix: handle empty config file
0f9e8d7 docs: update README
c0ffee1 perf(db): cache prepared statements
badc0de Merge branch main
7d6e5f4 feat(cli)!: rename --dir to --path
1a2b3c4 chore: bump dependencies
9f8e7d6 fix(api): return 404 for unknown users'

parse_commit() {
    local re='^([0-9a-f]+) ([a-z]+)(\(([a-z0-9-]+)\))?(!)?: (.+)$'
    [[ $1 =~ $re ]] || return 1
    c_hash=${BASH_REMATCH[1]}
    c_type=${BASH_REMATCH[2]}
    c_scope=${BASH_REMATCH[4]}
    c_breaking=${BASH_REMATCH[5]}
    c_subject=${BASH_REMATCH[6]}
}

next_version() {
    local major minor patch
    IFS=. read -r major minor patch <<< "$1"
    case $2 in
        major) echo "$(( major + 1 )).0.0" ;;
        minor) echo "$major.$(( minor + 1 )).0" ;;
        patch) echo "$major.$minor.$(( patch + 1 ))" ;;
    esac
}

declare -A sections=()
breaking="" skipped=() level=0
while IFS= read -r line; do
    if ! parse_commit "$line"; then
        skipped+=("$line")
        continue
    fi
    entry="- ${c_scope:+**$c_scope:** }$c_subject ($c_hash)"
    if [[ -n $c_breaking ]]; then
        breaking+="$entry"$'\n'
        level=2
    fi
    case $c_type in
        feat) (( level < 1 )) && level=1 ;;
        docs|chore|ci|test) continue ;;
    esac
    sections[$c_type]+="$entry"$'\n'
done <<< "$commits"

names=(patch minor major)
bump=${names[level]}
next=$(next_version "$current" "$bump")
echo "bump: $bump ($current -> $next)"
echo
echo "## v$next (2026-11-03)"
if [[ -n $breaking ]]; then
    echo
    echo "### ⚠ Breaking changes"
    printf '%s' "$breaking"
fi
for pair in "feat:Features" "fix:Bug fixes" "perf:Performance"; do
    type=${pair%%:*}
    [[ -n ${sections[$type]:-} ]] || continue
    echo
    echo "### ${pair#*:}"
    printf '%s' "${sections[$type]}"
done
echo
for s in "${skipped[@]}"; do echo "skipped (not conventional): $s"; done

@@exercise Pre-flight checks and a dry run
Write `release BRANCH STATUS TAGS NEXT` (the git outputs are passed in as strings so the exercise runs without a repository):

- fail with a clear message if the branch isn't `release`, if the status (from `git status --porcelain`) isn't empty (show the number of changed files), or if the tag `vNEXT` already exists in TAGS
- otherwise print the commands it would run in dry-run mode through a `run` function that uses `%q`: tag, push the branch, push the tag
- return 0 on success and 1 on any failed check, and collect **all** failed checks before returning (not just the first)
@@starter
echo TODO
@@expected
== release v2.0.0
[dry-run] git tag -a v2.0.0 -m Release\ v2.0.0
[dry-run] git push origin release
[dry-run] git push origin v2.0.0
result: 0
== release v1.4.2
error: tag v1.4.2 already exists
result: 1
== release v2.0.0
error: on branch 'main', releases must come from 'release'
error: working tree has 2 uncommitted change(s)
result: 1
@@requires
run()
%q
--porcelain
@@hint
Count changed files with `mapfile -t changes <<< "$status"` (but an empty string gives one empty item, so check `[[ -n $status ]]` first). Check the tag with `grep -qx "v$next" <<< "$tags"`. Accumulate `errors+=("...")` and print them all at the end.
@@solution
dry_run=1

run() {
    if (( dry_run )); then
        printf '[dry-run]'
        printf ' %q' "$@"
        echo
    else
        "$@"
    fi
}

release() {
    local branch=$1 status=$2 tags=$3 next=$4
    local -a errors=() changes=()
    echo "== release v$next"
    [[ $branch == release ]] || errors+=("on branch '$branch', releases must come from 'release'")
    if [[ -n $status ]]; then
        mapfile -t changes <<< "$status"
        errors+=("working tree has ${#changes[@]} uncommitted change(s)")
    fi
    grep -qx "v$next" <<< "$tags" && errors+=("tag v$next already exists")
    if (( ${#errors[@]} )); then
        printf 'error: %s\n' "${errors[@]}"
        return 1
    fi
    run git tag -a "v$next" -m "Release v$next"
    run git push origin "$branch"
    run git push origin "v$next"
}

# simulated outputs of: git rev-parse --abbrev-ref HEAD / git status --porcelain / git tag
tags=$'v1.3.0\nv1.4.0\nv1.4.2'
release release "" "$tags" 2.0.0; echo "result: $?"
release release "" "$tags" 1.4.2; echo "result: $?"
release main $' M src/app.sh\n?? notes.txt' "$tags" 2.0.0; echo "result: $?"
