use std::env;
use std::fs;

// remove this build file on release
fn main() {
    println!("cargo:rerun-if-changed=src/docs.md");
    println!("cargo:rerun-if-changed=src/feature-adaptive-docs.md");

    let script = env::var("CARGO_FEATURE_SCRIPT").is_ok();

    // remove on release
    let yew_next = env::var("CARGO_FEATURE_YEW_NEXT").is_ok();
    // remove on release
    let yew_stable = env::var("CARGO_FEATURE_YEW_STABLE").is_ok();

    let mut readme_doc_test = String::new();

    let content = include_str!("src/docs.md");

    let lines = content.split('\n');

    let mut is_fence = false;
    let mut fence: String = String::new();
    lines.for_each(|line| {
        if line.starts_with("```rust") {
            is_fence = true;
            fence.push_str(line);
            fence.push('\n');
            // remove if else if else on release
            if yew_next {
                fence.push_str("# extern crate yew_master as yew;\n")
            } else if yew_stable {
                fence.push_str("# extern crate yew_19 as yew;\n")
            } else {
                panic!("one of the features yew-stable or yew-next has to be enabled")
            }
        } else if line.starts_with("```") && is_fence {
            if script || !fence.contains("# //script") {
                readme_doc_test.push_str(&fence);
                readme_doc_test.push_str(line);
            }
            is_fence = false;
            fence.clear();
        } else if is_fence {
            fence.push_str(line);
            fence.push('\n');
        } else {
            readme_doc_test.push_str(line);
            readme_doc_test.push('\n')
        }
    });

    fs::write("src/feature-adaptive-docs.md", readme_doc_test).unwrap();
}
