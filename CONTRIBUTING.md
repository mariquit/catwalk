# Contributing to Catwalk

Use **Catwalk** in prose and `catwalk` for the command and guidance skill. Follow the
[Code of Conduct](CODE_OF_CONDUCT.md) in project spaces.

## Licensing of contributions

Catwalk is licensed under the [Apache License, Version 2.0](LICENSE). By submitting
a contribution, you agree to license it under the same terms. **There is no
CLA.** You retain copyright to your work.

### Sign your work

Catwalk uses the [Developer Certificate of Origin 1.1](DCO). Sign off each commit
with `git commit -s` to certify that you have the right to submit it. Verify
that the commit message contains this trailer with your real name and a working
email address:

```text
Signed-off-by: Your Name <your.email@example.com>
```

For an existing commit on your own unpublished branch, use
`git commit --amend -s`, or `git rebase --signoff <base>` for multiple commits.

### Copyright headers and prior art

Use the existing SPDX and copyright header in new source files:

```rust
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 John Anthony Mariquit <john@mariquit.com>
```

In shell scripts, use `#` comments after the shebang. In `assets/SKILL.md`,
keep the notices as YAML comments immediately below the opening `---` so the
frontmatter stays first and the notices remain outside the skill body.

You may add your own copyright notice to files you substantially author.
Preserve applicable notices and mark modified files as required by Apache-2.0
§4. Do not use proprietary, unlicensed, or license-incompatible material.
Identify compatible prior art in the pull request and record required
attribution in [NOTICE](NOTICE). See [TRADEMARK.md](TRADEMARK.md) for naming
and attribution guidance.

## Build and test

Use stable Rust and Git. Run the following checks from the repository root
before opening a pull request. Each command must exit successfully.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo test --locked
```

To inspect TUI changes, run `cargo run --locked -- --file examples/TODO.md`.
Use a copy of the example when testing annotation writes. Check that the note
appears in the TUI and the saved diff changes only the selected annotation.

## Architecture and scope

Read [AGENTS.md](AGENTS.md) and [WORKFLOW.md](WORKFLOW.md) before changing
file handling or the agent handoff. `src/lib.rs` parses the board and rewrites
only the trailing annotation of a uniquely identified task. `src/tui.rs` uses
`notify`, `ratatui`, and `crossterm` to watch and display the file. Viewing and
reloading must never write the board. Do not add task mutation, lifecycle
enforcement, or Git integration. Avoid blocking operations in the render loop
and large dependencies.

Update `src/lib.rs`, `README.md`, and `assets/SKILL.md` together when changing
task notation. Edit the skill here, not in an installed copy. Rebuild and
rerun `catwalk setup <agent>` to verify that the installed skill matches
`assets/SKILL.md`.

## Commits and pull requests

Use Conventional Commits, such as `fix(parser): preserve CRLF in annotations`
or `docs(workflow): clarify who edits task priority`. Explain why the change is
needed and how it was verified. Keep pull requests focused, test new behavior,
and omit secrets and machine-specific paths.

## GitHub releases

Set the Cargo package version in `Cargo.toml` and refresh the Catwalk entry in
`Cargo.lock`. Run the checks above and confirm that `cargo run --locked --
 --version` prints `catwalk vX.Y.Z` for the intended version. A push of a matching
`vX.Y.Z` tag triggers the [release workflow](.github/workflows/release.yml),
which builds platform binaries, generates SHA-256 checksums, and publishes both
to GitHub Releases. Check that the release contains a checksum for each binary.

The existing `v0.1.0` tag refers to an earlier commit. Setting the source
version to `0.1.0` does not replace that tag or publish a new release. Do not
reuse a release tag without a separate release decision.

## Reporting bugs

Check [existing issues](https://github.com/mariquit/catwalk/issues), then report
the `catwalk --version` output, operating system, terminal emulator, relevant Rust
version for source builds, and a minimal sanitized `TODO.md` snippet. Report
suspected vulnerabilities privately as described in [SECURITY.md](SECURITY.md).
