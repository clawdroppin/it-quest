@@lesson d0-git-internals Git Internals: Objects, Refs and the Commit Graph
@@text
Git feels mysterious until you see how simple its data model is. Everything is stored as **content-addressed objects** in `.git/objects`, branches are just **files containing a commit hash**, and almost every command is a walk over the **commit graph**. This lesson explains objects, refs, the index, merge bases and how merge, rebase and reset look from the inside, and the exercises compute a real Git blob hash and find a merge base.

## Objects

| Object | Contains |
|---|---|
| **blob** | file contents (no name, no permissions) |
| **tree** | a directory: entries of mode, name and blob/tree hash |
| **commit** | a tree hash, parent hash(es), author, committer, message |
| **tag** (annotated) | an object hash, tagger, message, optional signature |

- An object's ID is the **SHA-1** (or SHA-256 in newer repositories) of `"<type> <size>\0" + content`. Identical content → identical ID, so Git stores each file version once.
- Changing one file creates a new blob, new trees up to the root and a new commit. Unchanged files are reused by hash.
- Objects are zlib-compressed loose files, later packed into **packfiles** with delta compression (`git gc`).
- Inspect with `git cat-file -t <hash>` (type) and `git cat-file -p <hash>` (pretty print); `git hash-object file` computes a blob hash.

## Refs and HEAD

- A branch is a file such as `.git/refs/heads/main` containing a commit hash (or a line in `packed-refs`). Committing moves the current branch forward.
- **HEAD** usually points to a branch (`ref: refs/heads/main`). **Detached HEAD** means HEAD contains a commit hash directly; new commits are reachable only from HEAD, so create a branch before switching away.
- Remote-tracking branches (`origin/main`) are refs updated by `fetch`. Tags are refs to commits or tag objects.
- The **reflog** records where each ref pointed; it rescues "lost" commits after a bad reset or rebase (`git reflog`, then `git branch rescue <hash>`).

## The three trees

**HEAD** (last commit), the **index** (staging area: the next commit's snapshot) and the **working tree** (your files). `git add` copies working tree → index; `git commit` turns the index into a tree and commit.
- `git reset --soft X`: move the branch to X; keep index and working tree.
- `git reset --mixed X` (default): also reset the index.
- `git reset --hard X`: also overwrite the working tree. Uncommitted changes are lost.

## The commit graph

- Commits form a **DAG** (directed acyclic graph) through parent links. A merge commit has two (or more) parents.
- The **merge base** is the best common ancestor of two commits (`git merge-base a b`). A three-way merge compares both sides with the merge base.
- **Fast-forward**: if the target is an ancestor of the source, the branch just moves; no merge commit.
- **Rebase** replays your commits onto a new base, creating **new commits with new hashes** (the parent changed). Never rebase commits others have already pulled.
- `A..B` means commits reachable from B but not from A (what B has that A doesn't); `git log main..feature`.

@@quiz
What does a Git blob store?
* File contents only
- File name and contents
- A directory listing
- A commit message
? Names live in trees.

@@quiz
What is a branch in Git's data model?
- A copy of the files
* A ref: a file containing a commit hash
- A tree object
- A packfile
? Cheap to create and move.

@@quiz
After rebasing three commits onto a new base, what happens to their hashes?
- They stay the same
* They change, because each commit's parent changed
- Only the last one changes
- They become merge commits
? The commit content includes its parent hash.

@@quiz
Which reset keeps your changes staged in the index?
* `git reset --soft`
- `git reset --mixed`
- `git reset --hard`
- `git reset --keep`
? Soft moves only the branch.

@@quiz
`git log main..feature` lists what?
- Commits on main not on feature
* Commits reachable from feature but not from main
- All commits on both
- Merge commits only
? Two-dot range.

@@quiz
You ran `git reset --hard` too far back. How do you find the lost commit?
- It's gone forever
* `git reflog` shows where the branch pointed before
- `git fsck --delete`
- `git clone` again
? Reflog entries last about 90 days by default.

@@flashcards
blob :: file contents
tree :: directory entries
commit :: tree + parents + metadata
annotated tag :: tag object with message
object ID :: hash of "type size\0content"
packfile :: compressed, delta-encoded objects
branch :: ref to a commit
HEAD :: current branch or commit
detached HEAD :: HEAD holds a hash
reflog :: history of ref positions
index :: staging area
reset --soft :: move branch only
reset --mixed :: move branch, reset index
reset --hard :: also overwrite files
merge base :: best common ancestor
fast-forward :: move branch, no merge commit
rebase :: replay commits, new hashes
A..B :: in B, not in A

@@drill
Which command pretty-prints a Git object by its hash?
= git cat-file -p
= cat-file -p
? -t shows the type.

@@drill
Which command shows the best common ancestor of two commits?
= git merge-base
= merge-base
? Used by three-way merges.

@@exercise Compute a Git object ID
Compute Git blob IDs the way `git hash-object` does: SHA-1 of `"blob <size>\0"` followed by the content bytes. Show that identical content gives the same ID wherever it lives, that one changed byte gives a completely different ID, and check the well-known ID of the empty blob.
@@lang python
@@starter
import hashlib

files = {
    "README.md": b"# Demo\n",
    "docs/README.md": b"# Demo\n",
    "app.py": b"print('hello')\n",
    "app_v2.py": b"print('hello!')\n",
    "empty.txt": b"",
}

def blob_id(content):
    raise NotImplementedError
@@expected
0805455a24b6c68fbc38d0fa5d121f735984285d  README.md (7 bytes)
0805455a24b6c68fbc38d0fa5d121f735984285d  docs/README.md (7 bytes)
b376c9941fda362c8d2c5c8ddb35db3e0b003402  app.py (15 bytes)
0df117e334e7fc0f081465177bf3aca40ef57a5b  app_v2.py (16 bytes)
e69de29bb2d1d6434b8b29ae775ad8c2e48c5391  empty.txt (0 bytes)
stored objects: 4 for 5 files
shared blob 0805455: README.md, docs/README.md
one byte changed, matching hex digits: 2 of 40
empty blob ok: True
@@requires
def blob_id
hashlib.sha1(
b"\0"
@@hint
`header = f"blob {len(content)}".encode() + b"\0"`; then `hashlib.sha1(header + content).hexdigest()`. Group files by ID to find duplicates. The empty blob is `e69de29bb2d1d6434b8b29ae775ad8c2e48c5391`.
@@solution
import hashlib

files = {
    "README.md": b"# Demo\n",
    "docs/README.md": b"# Demo\n",
    "app.py": b"print('hello')\n",
    "app_v2.py": b"print('hello!')\n",
    "empty.txt": b"",
}

def blob_id(content):
    header = f"blob {len(content)}".encode() + b"\0"
    return hashlib.sha1(header + content).hexdigest()

ids = {}
for name, content in files.items():
    oid = blob_id(content)
    ids.setdefault(oid, []).append(name)
    print(f"{oid}  {name} ({len(content)} bytes)")
print("stored objects:", len(ids), "for", len(files), "files")
for oid, names in ids.items():
    if len(names) > 1:
        print(f"shared blob {oid[:7]}: {', '.join(names)}")
a, b = blob_id(files["app.py"]), blob_id(files["app_v2.py"])
print("one byte changed, matching hex digits:", sum(x == y for x, y in zip(a, b)), "of 40")
print("empty blob ok:", blob_id(b"") == "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")

@@exercise Merge base and ranges in a commit graph
Given a commit DAG (commit → parents), compute the ancestors of a commit, the merge base of two branches (a common ancestor that isn't an ancestor of another common ancestor), whether merging would fast-forward, and the `A..B` range: commits in B but not in A.
@@lang python
@@starter
parents = {
    "a1": [], "a2": ["a1"], "a3": ["a2"],
    "f1": ["a2"], "f2": ["f1"],
    "a4": ["a3", "f2"],
    "f3": ["f2"], "f4": ["f3"],
    "h1": ["a4"],
}
branches = {"main": "a4", "feature": "f4", "hotfix": "h1"}

def ancestors(commit):
    raise NotImplementedError
@@expected
main..feature: f3, f4; merge base f2; three-way merge needed
main..hotfix: h1; merge base a4; fast-forward main to hotfix
feature..hotfix: a3, a4, h1; merge base f2; three-way merge needed
@@requires
def ancestors
def merge_base
fast-forward
@@hint
`ancestors` includes the commit itself; use a stack. Common = ancestors(a) & ancestors(b); the best ones are those not an ancestor of another common commit (exclude c if c is in ancestors(other) - {other}). Fast-forward when one tip is an ancestor of the other.
@@solution
parents = {
    "a1": [], "a2": ["a1"], "a3": ["a2"],
    "f1": ["a2"], "f2": ["f1"],
    "a4": ["a3", "f2"],
    "f3": ["f2"], "f4": ["f3"],
    "h1": ["a4"],
}
branches = {"main": "a4", "feature": "f4", "hotfix": "h1"}

def ancestors(commit):
    seen, stack = set(), [commit]
    while stack:
        c = stack.pop()
        if c not in seen:
            seen.add(c)
            stack.extend(parents[c])
    return seen

def merge_base(a, b):
    common = ancestors(a) & ancestors(b)
    return sorted(c for c in common
                  if not any(c in ancestors(o) - {o} for o in common))

def range_(a, b):
    return sorted(ancestors(b) - ancestors(a))

for x, y in (("main", "feature"), ("main", "hotfix"), ("feature", "hotfix")):
    ta, tb = branches[x], branches[y]
    base = merge_base(ta, tb)
    if ta in ancestors(tb):
        how = f"fast-forward {x} to {y}"
    elif tb in ancestors(ta):
        how = f"{y} is already in {x}"
    else:
        how = "three-way merge needed"
    print(f"{x}..{y}: {', '.join(range_(ta, tb)) or '(nothing)'}; merge base {', '.join(base)}; {how}")
