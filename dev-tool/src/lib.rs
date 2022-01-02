use std::process::Stdio;

pub use std::process::Command;

pub trait Task {
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
