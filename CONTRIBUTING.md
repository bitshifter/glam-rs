# Contributing to glam

Thanks for contributing to `glam`! These guidelines will try to make the
process painless and efficient.

The short guide to contributing is [start a discussion] on GitHub.  Pull
requests are welcome for bug fixes, documentation improvements and
optimizations. For anything else it would be best to discuss it first.

## Questions

If you have a question about the usage of this library please [ask a question]
with GitHub Discussions. That's the easiest way to get support right now.

## Bugs

If you find a bug please [open an issue] on GitHub or submit a pull request. A
unit test for any bug that slipped through existing coverage would also be
greatly appreciated.

## New functions and methods

If `glam` is missing functionality on existing types, [suggest a new feature]
with GitHub Discussions describing what feature you would like added and
ideally what your use case is for it just so I have a better understanding of
the feature. I'd like to keep `glam` reasonably light functionality wise
initially but commonly used functionality that is missing is very welcome. If
you do submit a pull request please ensure any new functionality also has a
test.

## Optimizations

If you feel some functionality could be optimized please [open an issue] on
GitHub or submit a pull request. Any optimization pull request should include a
benchmark if there isn't one already, so I can confirm the performance
improvement.

## Documentation

If you feel any documentation could be added or improved please
[open a GitHub issue] or submit a pull request.

## Pull request titles

PR titles should follow the [Conventional Commits] format, since the
squash-merged title becomes the commit message that is used to generate the
[changelog] and determine the version bump when a release is prepared with
release-plz:

```
<type>(<scope>): <description>
```

Common types are `feat`, `fix`, `refactor`, `perf`, `docs`, `test`,
`chore`, `ci`, `build` and `revert`. A scope describing the affected code
area is optional, e.g. `feat(quat): ...` or `fix(vec3): ...`.

Breaking changes append `!` after the type or scope, e.g.
`feat(quat)!: remove the deprecated camera methods`. Breaking changes
bump the minor version at the next release while glam is pre-1.0; they
are also detected automatically by cargo-semver-checks when the release
PR is prepared, so a minor bump can occur even without the `!` marker.

A check on the PR will suggest this format for titles that don't follow it.
The check is not required and titles can also be adjusted in the merge
dialog when squashing; dependabot, release-plz and draft PRs are exempt.

## Code contributions

Most of `glam`'s source code is generated. See the [codegen README] for how to
modify the code templates and generate new source code.

Edit templates in the `templates/` directory (they use the [Tera v2] templating
language) and the `codegen.json` file which maps templates to output files.
Generated files are identified by the header comment at the top of the file,
e.g. `// Generated from vec.rs.tera template. Edit the template, not the
generated file.`

After modifying templates, run `cargo run --release -p codegen` from the repo
root to regenerate source files (requires initializing the codegen submodule
with `git submodule update --init tools/codegen`). By default codegen skips
output files that have uncommitted modifications. Pass `-f` (or `--force`) to
overwrite them, which is usually what you want when iterating on a template. A
glob argument limits regeneration to matching files for faster iteration, e.g.
`cargo run --release -p codegen -- -f 'src/f32/vec3.rs'`. Generated files are
already rustfmt-formatted, so `cargo fmt` is only needed for hand-written files
such as tests.

The minimum supported Rust version is 1.68.2 and is checked by
`cargo run -p ci -- msrv`, so avoid using newer language features in code or
tests.

You can run `glam`'s test suite locally:

- `cargo test` runs everything, or `cargo test --test vec3` for a single test
  file.
- Some tests assert that `glam_assert!` panics on invalid input; these only
  take effect with the `glam-assert` or `debug-glam-assert` feature enabled, so
  run `cargo test --features=debug-glam-assert` to check them.
- `cargo run -p ci` runs the same checks as the pre-push hook (fmt, clippy and
  tests across feature combinations). It's worth running that before creating a
  PR. The fuller `cargo run -p ci -- ci` suite additionally checks the MSRV and
  wasm targets and needs nightly and wasm toolchains installed, so it's usually
  best left to GitHub Actions.

Also run `cargo fmt` on any new hand-written files and `cargo clippy` on any new code.

[start a discussion]: https://github.com/bitshifter/glam-rs/discussions/new
[open an issue]: https://GitHub.com/bitshifter/glam-rs/issues/new
[ask a question]: https://github.com/bitshifter/glam-rs/discussions/new?category=q-a
[suggest a new feature]: https://github.com/bitshifter/glam-rs/discussions/new?category=ideas
[codegen README]: https://github.com/bitshifter/glam-codegen/blob/main/README.md
[Tera v2]: https://keats.github.io/tera/
[Conventional Commits]: https://www.conventionalcommits.org/
[changelog]: CHANGELOG.md
