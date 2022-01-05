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

pub fn generate_readme() {
    let mut output = String::new();

    let content = std::fs::read_to_string("yew-interop/src/docs.md").unwrap();

    let lines = content.split('\n');

    let mut is_fence = false;
    let mut fence: String = String::new();
    lines.for_each(|line| {
        if line.starts_with("```rust") {
            is_fence = true;
            fence.push_str("```rust\n");
        } else if line.starts_with("```") && is_fence {
            output.push_str(&fence);
            output.push_str(line);
            is_fence = false;
            fence.clear();
        } else if is_fence {
            if !line.starts_with("# ") || line == "#" {
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
