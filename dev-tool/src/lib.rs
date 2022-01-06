use std::process::{ExitStatus, Stdio};

pub use std::process::Command;

/// A task is a command which inherits stdout and stderr from the the standard io.
pub trait Task {
    /// like run but with an ending note
    fn run_with_note(&mut self, note: &'static str) -> ExitStatus;
    /// returns true when success (zero exit code)
    fn run(&mut self) -> ExitStatus;
}

fn get_status(command: &mut Command) -> ExitStatus {
    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap()
        .status
}

impl Task for Command {
    fn run_with_note(&mut self, note: &'static str) -> ExitStatus {
        let status = get_status(self);
        println!("{}", note);
        status
    }
    fn run(&mut self) -> ExitStatus {
        get_status(self)
    }
}

// simplified version of cargo-expand
// expects `cargo rustc` and `rustfmt` to exist
pub fn generate_readme() {
    let mut builder = tempfile::Builder::new();
    builder.prefix("yew-interop-readme-gen");
    let outdir = builder.tempdir().expect("failed to create tmp file");
    let outfile_path = outdir.path().join("expanded");
    let cmd = Command::new("cargo")
        .args([
            "+nightly",
            "rustc",
            "--features",
            "yew-stable",
            "--features",
            "script",
            "-p",
            "yew-interop",
            "--",
            "-Zunpretty=expanded",
            "-o",
            outfile_path.to_str().unwrap(),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::null())
        .output()
        .unwrap()
        .status;
    assert!(
        cmd.success(),
        "rustc failed to expand yew-interop/src/lib.rs"
    );

    let cmd = Command::new("rustfmt")
        .args([
            outfile_path.to_str().unwrap(),
            "--edition=2021",
            "--config",
            "normalize_doc_attributes=true",
        ])
        .run();

    assert!(cmd.success(), "rustfmt failed to normalize doc strings");

    let mut output = String::new();
    let content = std::fs::read_to_string(&outfile_path).unwrap();

    let lines = content.split('\n');

    let mut is_fence = false;
    let mut fence: String = String::new();
    lines
        .filter_map(|line| line.strip_prefix("//!"))
        .for_each(|line| {
            if line.starts_with("```rust") {
                is_fence = true;
                fence.push_str("```rust\n");
            } else if line.starts_with("```") && is_fence {
                output.push_str(&fence);
                output.push_str(line);
                output.push('\n');
                is_fence = false;
                fence.clear();
            } else if is_fence {
                if !line.starts_with("# ") && line != "#" {
                    fence.push_str(line);
                    fence.push('\n');
                }
            } else {
                output.push_str(line);
                output.push('\n')
            }
        });

    std::fs::write("README.md", output).unwrap();
}
