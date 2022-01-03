use std::fs;

// remove this build file on release
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=../README.md");

    let mut readme_doc_test = String::new();

    let content = fs::read("../README.md").unwrap();
    let content = String::from_utf8(content).unwrap();

    let lines = content.split('\n');

    let mut capture = false;
    let mut starting_no: usize = 0;
    let mut fence: String = String::new();
    lines.enumerate().for_each(|(line_no, line)| {
        if line.starts_with("```rust") && !line.contains("ignore") && !line.contains("norun") {
            starting_no = line_no;
            capture = true;
        } else if line.starts_with("```") && capture {
            if fence.contains("script feature") {
                readme_doc_test.push_str("#[cfg(feature = \"script\")]\n");
            }
            readme_doc_test.push_str(&format!(
                "#[allow(dead_code)]\nmod line{}to{}\n {{{}}}\n\n",
                starting_no, line_no, fence
            ));
            capture = false;
            fence.clear();
        } else if capture {
            fence.push_str(line);
            fence.push('\n');
        }
    });

    fs::write("src/readme.rs", readme_doc_test).unwrap();
}
