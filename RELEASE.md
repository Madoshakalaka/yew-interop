# Before a New Release of Yew Interop

Let's suppose the previous version is `4.3.1` and you are publishing `4.3.2`.

0. Fix yew version in each crate to stable.

1. use `dev-tool/bin/build-demo.rs` to build a new website with the new version.
Example: `cargo run -p dev-tool --bin build-demo -- 4.3.2`.
Push it to master so the new site is deployed.
This will deploy the versioned website and the versioned static files.
So that other documentation can refer to them.

2. checkout a new branch from master with the v+ the new version, e.g. `v4.3.2`

3. search for links in the documentation for the previous version, change with the new version.

    e.g. the crate level documentation might have a link that refers to an old GitHub branch.

    Recommendation : 
   1. cd to `yew-interop` and run `ag 4.3.1`. (replace 4.3.1 with the previous version string)
   2. search in `README.md` for links with "master",
   replace them with the links of the versioned static files.

4. Remove the yew-next and yew-stable features and dependencies.
Use stable yew instead.

# After a Release

Tag the branch and delete the branch.