# What Doesn't Work

- `cargo check/build/clippy/fix` at the workspace root doesn't enable features correctly because the use of mutually
  exclusive features in the `yew-interop` crate and its interaction with the `example` crate.

- `cargo-make` is not used here.

# What Works

- `cargo fmt` at the workspace root
- `cargo check/build/clippy/fix -p <package_name> ...`

As an alternative, The [the dev-tool](dev-tool/src/bin) crate has everything you need.

# Before a PR

> pre-pr is not yet implemented

`cargo run -p dev-tool -- pre-pr`

This will run every test possible.

# If Your PR to Master Touched the [`example`](example) Crate Or Its Dependencies

A GitHub workflow will rebuild the demo website, if the output is different from the previous website, a bot will push
it to your PR shortly under [`docs/master`](docs/master). Don't panic and merge the change if you have followup commits.

It allows you a chance to preview the built demo website locally and see if anything goes wrong.

# If Your PR to master Touched the [`yew-interop`](yew-interop) Crate Or Its Dependencies

> The workflow is not yet there

A GitHub workflow will rebuild the docs for master, if the output is different from the previous website, a bot will
push it to your PR shortly under [`docs/docsrs`](docs/docsrs). Don't panic and merge the change if you have followup
commits.

It allows you to preview the resulting docs.rs page locally