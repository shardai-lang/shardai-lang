Big thanks for your interest in contributing to Shardai!

> [!WARNING]
> Shardai is currently in very early development, and the internals change frequently.
> Before you contribute, note that core systems like the bytecode format, AST structure, VM design, etc. may be redesigned without notice until 1.0.

## Ways to contribute:
* Bug reports: "found a crash," unexpected behavior, incorrect output
* Bug fixes: small, doesn't touch core language design
* Discussion: feedback on opened issues
* Documentation: improving the README, adding code comments, etc.

> [!WARNING]
> Large features or redesigns of core systems aren't being accepted as the language design isn't stable yet.
> If you have ideas, please open an issue first.

## Bug reports:
These need to be in your report:
* What did you do?
* What did you expect to happen?
* What actually happened?
* Minimal code snippet for reproducing the bug

## Building from source:
1. Install Rust (https://rustup.rs)
2. Clone the repo:
```bash
git clone https://github.com/shardai-lang/shardai-lang
cd shardai-lang
```
3. Build:
```bash
cargo build
```
4. Done!

## Code style:
* Run `cargo fmt` before committing; the CI will automatically reject unformatted code
* Run `cargo clippy` and resolve any warnings before submitting a PR

## Commit messages:
1. Keep the summary short (under 80 characters)
2. Be imperative: "add closure support" not "added closure support"
3. Reference the issue number if one exists "fix bla bla bla, closes #4"

## Pull requests process:
1. Open a PR against the master branch with a description of what it does, and why
2. Link the issue it addresses if one exists
3. CI must pass before review
4. One approval from a maintainer is required to merge
5. Once merged, delete your branch

For anything more than a small bug fix, open an issue and discuss your approach before writing code. This avoids wasted effort if the approach doesn't fit the project
