# What Doesn't Work

- `cargo check/build/clippy/fix` at the workspace root doesn't enable features correctly because the use of
mutually exclusive features in the `yew-interop` crate and its interaction with the `example` crate.

- `cargo-make` is not used here.

# What Works

- `cargo fmt` at the workspace root
- `cargo check/build/clippy/fix -p <package_name> ...`
- The [bin folder of the dev-tool](dev-tool/src/bin) crate has everything you
need. These will also run in GitHub Actions.

# Before a PR

> pre-pr is not yet implemented

`cargo run -p dev-tool --bin pre-pr`

This will run every test in the [`dev-tool`](dev-tool) crate and also run cargo fmt.

# If Your PR Touched `dev-tool`

A GitHub workflow will rebuild the executables, 
if they are different, a bot will and push them to your PR shortly under [`dev-tool/ci-bin`](dev-tool/ci-bin).
Don't panic and merge the change if you have followup commits.

# If Your PR Forgot to Lint the Code

A GitHub workflow will lint the code,
if it results in a difference,
a bot will push them to your PR.
Don't panic and merge the change if you have followup commits.
