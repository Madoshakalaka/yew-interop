# Before a New Release of Yew Interop

Let's suppose the previous version is `4.3.1` and you are publishing `4.3.2`.


1. Dispatch the publish-demo workflow with the new version `4.3.2`.
This will deploy the versioned website and the versioned static files.
So that other documentation can refer to them.

2. checkout a new branch from master with the v+ the new version, e.g. `v4.3.2`
3. Fix yew version to stable in each yew crate

4. grep for "on release" globally and change them accordingly

5. uncomment the relative dependencies just to do the tests below.

6. do `cargo clippy`, `cargo test --all-targets`, `cargo clippy --features script` `cargo test --features script`
   and `cargo fmt`

7. do `RUSTDOCFLAGS="--cfg documenting" cargo +nightly doc -p yew-interop --all-features --no-deps --open`
to preview the docs


# After a Release

Tag the branch, do not delete the branch, 
for some of them tree is referenced in the readme published on crates.io.

On master, search for "after release" in `yew-interop/src/lib.rs` and update them.