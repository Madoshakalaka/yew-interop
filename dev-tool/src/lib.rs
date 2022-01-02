use std::process::Stdio;

pub use std::process::Command;

/// A task is a command which inherits stdout and stderr from the the standard io.
pub trait Task {
    /// will exit the process with the exit code from the command
    fn run(&mut self) -> !;
}

impl Task for Command {
    fn run(&mut self) -> ! {
        let status = self
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
            .status;

        std::process::exit(status.code().unwrap())
    }
}
