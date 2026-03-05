# Contributing to Hope:RE

We would love for you to contribute to this project and help make it even better!
As a contributor, here are the guidelines we would like you to follow:

- [Code of Conduct](#code-of-conduct)
- [Question or Problem?](#question-or-problem)
- [Issues and Bugs](#found-a-bug)
- [Feature Requests](#missing-a-feature)
- [Submission Guidelines](#submission-guidelines)
- [Coding Rules](#coding-rules)
- [Commit Message Format](#commit-message-format)

## Code of Conduct

Help us keep this project open and inclusive.
Please read and follow our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Question or Problem?

Do not open issues for general support questions as we want to keep GitHub issues for bug reports and feature requests.
Instead, we recommend reaching out directly via [email](mailto:trananhquan1009@gmail.com) for support-related questions.

## Found a Bug?

If you find a bug in the source code, you can help us by [submitting an issue](#submitting-an-issue) to our [GitHub Repository](https://github.com/HopeArtOrg/hope-re/issues).
Even better, you can [submit a Pull Request](#submitting-a-pull-request-pr) with a fix.

## Missing a Feature?

You can _request_ a new feature by [submitting an issue](#submitting-an-issue) to our GitHub Repository.
If you would like to _implement_ a new feature, please consider the size of the change:

- For a **Major Feature**, first open an issue and outline your proposal so that it can be discussed.
- **Small Features** can be crafted and [directly submitted as a Pull Request](#submitting-a-pull-request-pr).

## Submission Guidelines

### Submitting an Issue

Before you submit an issue, please search the issue tracker. An issue for your problem may already exist and the discussion might provide you with workarounds.

We want to fix all the issues as soon as possible, but before fixing a bug we need to reproduce and confirm it. In order to reproduce bugs, we require that you provide a minimal reproduction.

You can file new issues by selecting from our [new issue templates](https://github.com/HopeArtOrg/hope-re/issues/new/choose) and filling out the template.

### Submitting a Pull Request (PR)

Before you submit your Pull Request (PR), consider the following guidelines:

1. Search [GitHub PRs](https://github.com/HopeArtOrg/hope-re/pulls) for an open or closed PR that relates to your submission.
2. Make sure there is an issue describing the problem you are fixing, or documenting the design for the feature you would like to add.
3. [Fork](https://docs.github.com/en/github/getting-started-with-github/fork-a-repo) the repository.
4. In your forked repository, make your changes in a new git branch:

   ```sh
   git checkout -b my-fix-branch main
   ```

5. Create your patch.
6. Follow our [Coding Rules](#coding-rules).
7. Make sure to test your changes.
8. Run `pnpm lint` to ensure your code passes linting.
9. Commit your changes using a descriptive commit message that follows our [commit message conventions](#commit-message-format).
   ```sh
   git commit --all
   ```
10. Push your branch to GitHub:
    ```sh
    git push origin my-fix-branch
    ```
11. On GitHub, send a pull request to the `main` branch.

#### Reviewing a Pull Request

The maintainers reserve the right to not accept pull requests from community members who have not shown good conduct. Such behavior includes not following the [Code of Conduct](./CODE_OF_CONDUCT.md).

##### Addressing review feedback

If we ask for changes via code reviews, then:

1. Make the required updates to the code.
2. Create a fixup commit and push to your GitHub repository:
   ```sh
   git commit --all --fixup HEAD
   git push
   ```

##### Updating the commit message

A reviewer might suggest changes to a commit message. To update the message of the last commit on your branch:

1. Check out your branch:
   ```sh
   git checkout my-fix-branch
   ```
2. Amend the last commit and modify the commit message:
   ```sh
   git commit --amend
   ```
3. Push to your GitHub repository:
   ```sh
   git push --force-with-lease
   ```

#### After your pull request is merged

After your pull request is merged, you can safely delete your branch and pull the changes from the main repository:

- Delete the remote branch on GitHub via the GitHub web UI or your local shell:
  ```sh
  git push origin --delete my-fix-branch
  ```
- Check out the main branch:
  ```sh
  git checkout main -f
  ```
- Delete the local branch:
  ```sh
  git branch -D my-fix-branch
  ```
- Update your main branch with the latest upstream version:
  ```sh
  git pull --ff upstream main
  ```

## Coding Rules

To ensure consistency throughout the source code, keep these rules in mind as you are working:

- All features or bug fixes **must be tested**.
- Run `pnpm lint` before committing to ensure code passes ESLint checks.
- Follow Angular-style [commit message conventions](#commit-message-format).
- Use `type` keyword for TypeScript type definitions, never `interface`.
- Use `import type` for type-only imports.
- Use Svelte 5 runes (`$props()`, `$state()`, `$derived()`, `$effect()`) -- no legacy Svelte syntax.
- No `<style>` blocks in Svelte components -- use Tailwind CSS utility classes only.
- File names must be kebab-case (except Rust files which use snake_case).

## Commit Message Format

We have very precise rules over how our Git commit messages are formatted. This format leads to **easier to read commit history**.

Each commit message consists of a **header**, a **body**, and a **footer**.

```
<header>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

The `header` is mandatory and must conform to the format:

```
<type>(<scope>): <short summary>
```

- **type**: build | ci | docs | feat | fix | perf | refactor | test
- **scope**: the place of the commit change (e.g. `protection`, `commands`, `ui`, or empty)
- **summary**: imperative present tense, no capitalized first letter, no period at the end

The `body` is optional. When present, it should explain the motivation for the change and contrast this with previous behavior.

The `footer` is optional. It can contain information about breaking changes, deprecations, or references to issues and PRs.

### Revert commits

If the commit reverts a previous commit, it should begin with `revert: `, followed by the header of the reverted commit.
