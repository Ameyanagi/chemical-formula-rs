# Contribution Guidelines

## Issues

This repository contains issue templates for [bugs] and [feature requests].  
Unclear documentation or error messages are considered bugs.

For anything else, please use the ["Custom issue"] template.

[bugs]: https://github.com/Ameyanagi/chemical-formula-rs/issues/new?assignees=&labels=bug&template=bug_report.md&title=
[feature requests]: https://github.com/Ameyanagi/chemical-formula-rs/issues/new?assignees=&labels=enhancement&template=feature_request.md&title=
["Custom issue"]: https://github.com/Ameyanagi/chemical-formula-rs/issues/new?assignees=&labels=&template=custom_issue.md&title=

## Pull Requests

<!-- ### CI (TODO) -->
<!---->
<!-- This repository uses fairly extensive CI to make sure everything is in order.   -->
<!-- GitHub Actions will automatically build and test your pull requests. -->
<!---->
<!-- **I recommend working on branches with a `-` or `/` in their name.**   -->
<!-- The CI is configured slightly differently for them to make WIP code a bit easier. -->
<!---->
<!-- Additionally, when you run `cargo test` for the first time, [cargo-husky] sets up a Git pre-push hook to run tests.   -->
<!-- This includes a branch name check, which is ignored on any branches that have a `-` or '/' in their name.   -->
<!-- You can still push failing builds using `git push --no-verify`. -->
<!---->
<!-- Warnings are only denied on `develop`, but the CI should still detect them for pull requests towards that branch. -->
<!---->
<!-- [cargo-husky]: https://lib.rs/crates/cargo-husky -->

### Code Style

Please keep your code human-readable.

### Meta data

Please add yourself to each copyright holders list of [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) when contributing, or alternatively include a note in your pull request that you intentionally didn't do so.

Nicknames and entries without email addresses are fine, too.

For substantial contributions (basically anything more than typo or grammar fixes), feel free to add yourself to the `authors` list in `Cargo.toml`. This explicitly includes documentation changes, testing and bug fixes that just happen to be not much code.

### Optional: Update [CHANGELOG.md](CHANGELOG.md)

I use the following format for an upcoming release with contributed changes:

```markdown
## next

TODO: Date

- Revisions:
  - Change title (contributed by @<your GitHub @> in #<PR #>)
    > Further description or motivation, if necessary.
```

When adding your change, replace:

- `Revisions` with `Features` or `**Breaking changes**`, if your contribution falls into one of those categories instead.
- `<your GitHub @>` with your GitHub username.
- `<PR #>` with the id of your pull-request. (Squashing is optional)
- `Change title` and `> Further description…` as appropriate.

See non-contributed changes from earlier releases for examples.

## Labels

Don't worry about these too much.

You're encouraged to add any that seem applicable where possible,
but I'll otherwise just take care of it.

(Don't undo changes I make to labels without immediate reason.)

See <https://github.com/Ameyanagi/chemical-formula-rs/issues/labels> for details on individual labels.

### Categories

- Assorted

  Labels without prefix like [`breaking`](https://github.com/Ameyanagi/chemical-formula-rs/labels/breaking),
  [`good first issue`](https://github.com/Ameyanagi/chemical-formula-rs/labels/good%20first%20issue) or
  [`help wanted`](https://github.com/Ameyanagi/chemical-formula-rs/labels/help%20wanted).

- `domain:`

  Categorises changes by domain. Mostly not necessary.

- `effort:`

  Relative effort required. There's no specific unit of measurement.

- `priority:`

  Vaguely informs my schedule, **cross-repository**.

  You're welcome to let me know that (and ideally why) you'd like to see a specific change and I'll take that into account.

  <!-- If you _need_ a feature that you're not planning to implement yourself, strongly consider paying me for it. -->
  <!---->
  <!-- > This is, of course, subject to side-job restrictions I may be under. -->
  <!---->
  <!-- <!----> -->
  <!---->
  <!-- > If you'd like to pay me directly, contact me first and we'll figure out how to do this as industry-standard contract work. -->
  <!-- > -->
  <!-- > Alternatively: -->
  <!-- > -->
  <!-- > For crowdfunding and escrow, [Bountysource](https://www.bountysource.com/) seems reasonably trustworthy. This also has the advantage of letting someone else work on it, since I'm usually pretty swamped with projects. -->
  <!-- > -->
  <!-- > Use fiat bounties. Cryptoscam "currencies" are a scourge. -->
  <!-- > -->
  <!-- > Posting a bounty won't guarantee I'll actually implement a solution, but I'll try to speedily triage relevant issues at least. -->
  <!-- > -->
  <!-- > I'll try to set up something that automatically announces bounties, -->
  <!-- > but if that doesn't happen within a few hours, **do** post a comment about it! -->

- `state:`

  General scheduling categories. See label descriptions for details!

  Rarely, more than one may be applicable.

- `type:`

  General problem or change domain categories.

  Only one of these should be used at a time.

- `work:`

  These are inspired by the [Cynefin framework](https://en.wikipedia.org/wiki/Cynefin_framework) to categorise the type of work done or required.

  I adjusted the labels to be a bit friendlier and more expressive. See the label descriptions for details.

  The unknown category at the centre is not labelled explicitly.
