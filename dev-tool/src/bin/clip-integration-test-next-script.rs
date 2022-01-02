use dev_tool::*;

fn main() {
    Command::new("cargo")
        .args(["clippy", "-p", "integration-test", "--features", "yew-next", "--features", "script"])
        .run();
}
