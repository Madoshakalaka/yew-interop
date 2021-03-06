# What Doesn't Work

- `cargo check/build/clippy/fix --features ...` at the workspace root doesn't enable features correctly because the use of mutually
  exclusive features in the `yew-interop` crate and its interaction with the `example` crate.

# What Works

- `cargo fmt` at the workspace root
- `cargo check/build/clippy/fix -p <package_name> ...` at the workspace root (equivalently, cd to the package and run the command),
you can specify feature flags `yew-next`/`yew-stable`/`script`.
- `cargo check/build/clippy/fix` without `--features` flags at the workspace root.
Because of the said reason, this will enable `yew-stable` and `script` features for all crates automatically.

As an alternative, The [the dev-tool](dev-tool/src) crate has everything you need.
It is a CLI, run `cargo -p dev-tool -- -h` to see everything you can do.
And if you run it without arguments (`cargo run -p dev-tool`),
it will show an interactive menu with the tasks and checks you can run.

> the interactive menu has bad integration with JetBrains' Run window, 
> make sure to run it in a terminal

# Before a PR

`cargo run -p dev-tool -- pre-pr`

This will run every test possible.

# Workflow Generated Artifacts

## If Your PR to Master Touched the [`example`](example) Crate Or Its Dependencies

Upon merge to master, a GitHub workflow will rebuild the demo website, if the output is different from the previous website,
a bot will push it to master under [`docs/master`](docs/master).

If you wish to build and view the demo locally, you can run `cargo -p dev-tool -- serve-demo`

## If Your PR to master Touched the [`yew-interop`](yew-interop) Crate Or Its Dependencies

Upon merge to master, a GitHub workflow will rebuild the docs for master,
if the output is different from the previous website,
a bot will push it to the master branch under [`docs/docsrs`](docs/docsrs).

If you wish to build and view the docs locally, you can run `cargo -p dev-tool -- build-docs`

## Do Not Touch `README.md` Directly

It's generated from the module level documentation of [`yew-interop/lib.rs`](yew-interop/src/lib.rs).

Touch that one instead. 
Same as the cases above,
a workflow will build the readme and push to the master branch.

If you wish to generate the readme locally, you can run `cargo run -p dev-tool -- gen-readme`
