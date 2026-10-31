@@lesson xp-ci_pipeline CI/CD Pipeline for a Project
@@text
Take one of your projects from this app and give it a professional pipeline: on every push, lint and test it on Windows and Linux; on a tag, build it and publish a release. Add a status badge to the README and branch protection so broken code can't be merged.

**You'll practise:** GitHub Actions workflows · matrix builds · caching · artifacts and releases · branch protection

## Milestones

1. Pick a project and push it to a new GitHub repository.
2. **Workflow**: `on: [push, pull_request]`, a job that installs, lints and runs the tests.
3. **Matrix**: run on ubuntu-latest and windows-latest.
4. **Release**: on tags `v*`, build and upload the artifact to a GitHub Release.
5. **Protect**: require the checks to pass before merging into main.

## You're done when

- A pull request with a failing test is blocked, and pushing a tag creates a release.

## Getting help

Open the project, then the 🤖 **AI Helper**: it sees the lesson and your files. Good questions to start with:

- *"Review my workflow YAML for mistakes."*
- *"How do I cache pip or npm downloads in GitHub Actions?"*

Ask for hints, reviews and explanations rather than finished code: you learn by writing it.
@@project ci_pipeline CI/CD Pipeline for a Project
Create the folder, open it in VS Code, and work through `README.md` one milestone at a time.
@@file README.md
# CI/CD Pipeline for a Project

Take one of your projects from this app and give it a professional pipeline: on every push, lint and test it on Windows and Linux; on a tag, build it and publish a release. Add a status badge to the README and branch protection so broken code can't be merged.

## Milestones

- [ ] 1. Pick a project and push it to a new GitHub repository.
- [ ] 2. Workflow: `on: [push, pull_request]`, a job that installs, lints and runs the tests.
- [ ] 3. Matrix: run on ubuntu-latest and windows-latest.
- [ ] 4. Release: on tags `v*`, build and upload the artifact to a GitHub Release.
- [ ] 5. Protect: require the checks to pass before merging into main.

## Done when

- [ ] A pull request with a failing test is blocked, and pushing a tag creates a release.
@@file .github/workflows/ci.yml
name: ci
on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - run: python -m unittest -v
      # TODO: lint step, release job on tags
@@file test_example.py
import unittest


class Example(unittest.TestCase):
    def test_truth(self):
        self.assertEqual(1 + 1, 2)


if __name__ == "__main__":
    unittest.main()
