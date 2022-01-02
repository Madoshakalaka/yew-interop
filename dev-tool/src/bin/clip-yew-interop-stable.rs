use dev_tool::*;

fn main() {
    Command::new("cargo")
        .args(["clippy", "-p", "yew-interop", "--features", "yew-stable"])
        .run();
}
