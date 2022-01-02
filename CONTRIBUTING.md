# What Doesn't Work

- `cargo check/build/clippy/fix` at the workspace root doesn't enable features correctly because the use of
mutually exclusive features in the crates.

- `cargo-make` is not used here.

# What Works

- `cargo fmt` at the workspace root
- `cargo check/build/clippy/fix -p <package_name> ...`
- Instead of writing the commands yourself, the [bin folder of the dev-tool](dev-tool/src/bin) crate has everything you
need.




# Before a PR

- `cargo run -p dev-tool -bin pr-flow`

## If You Touched the dev-tool Crate


