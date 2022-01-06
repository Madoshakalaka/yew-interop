use clap::{Parser, Subcommand};
use derive_more::Display;
use dev_tool::Task;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::process::{Command, ExitStatus};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Parser)]
#[clap(about = "meant to be used locally by developers, please use at the workspace root")]
struct Args {
    /// omit this command for an interactive menu
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, EnumIter, Display, Copy, Clone)]
enum Commands {
    /// run all checks, short circuiting on any failure
    PrePr,
    /// generate README.md from yew-interop/src/docs.md, requires nightly
    GenReadme,
    /// serve the demo website under ./example using trunk
    ServeDemo,
    /// build and open the docs, need nightly rust.
    BuildDocs,
}

/// early return on a failure
fn any_fail(statuses: &[ExitStatus]) -> bool {
    statuses.iter().any(|x| !x.success())
}

impl Commands {
    fn run(&self) -> Result<(), ()> {
        let map_exit_status = |s: ExitStatus| s.success().then(|| ()).ok_or(());

        match self {
            Commands::PrePr => {
                let commands = &[
                    Command::new("cargo")
                        .args(["fmt", "--all", "--", "--check"])
                        .run(),
                    Command::new("cargo")
                        .args(["clippy", "--all-targets", "--", "-D", "warnings"])
                        .run(),
                    Command::new("cargo")
                        .args([
                            "clippy",
                            "-p",
                            "yew-interop",
                            "--features",
                            "yew-stable",
                            "--all-targets",
                            "--",
                            "-D",
                            "warnings",
                        ])
                        .run(),
                    Command::new("cargo")
                        .args([
                            "clippy",
                            "-p",
                            "yew-interop",
                            "--features",
                            "yew-next",
                            "--all-targets",
                            "--",
                            "-D",
                            "warnings",
                        ])
                        .run(),
                    Command::new("cargo")
                        .args([
                            "clippy",
                            "-p",
                            "yew-interop",
                            "--all-targets",
                            "--features",
                            "yew-next",
                            "--features",
                            "script",
                            "--",
                            "-D",
                            "warnings",
                        ])
                        .run(),
                    Command::new("cargo")
                        .args([
                            "clippy",
                            "-p",
                            "yew-interop-macro",
                            "--features",
                            "yew-stable",
                            "--",
                            "-D",
                            "warnings",
                        ])
                        .run(),
                    Command::new("cargo")
                        .args([
                            "clippy",
                            "-p",
                            "yew-interop-macro",
                            "--features",
                            "yew-next",
                            "--",
                            "-D",
                            "warnings",
                        ])
                        .run(),
                    Command::new("cargo")
                        .args([
                            "clippy",
                            "-p",
                            "yew-interop-macro",
                            "--features",
                            "yew-next",
                            "--features",
                            "script",
                            "--",
                            "-D",
                            "warnings",
                        ])
                        .run(),
                    Command::new("cargo").args(["test"]).run(),
                    Command::new("cargo")
                        .args(["test", "-p", "yew-interop", "--features", "yew-stable"])
                        .run(),
                    Command::new("cargo")
                        .args(["test", "-p", "yew-interop", "--features", "yew-next"])
                        .run(),
                    Command::new("cargo")
                        .args([
                            "test",
                            "-p",
                            "yew-interop",
                            "--features",
                            "yew-next",
                            "--features",
                            "script",
                        ])
                        .run(),
                ];
                (!any_fail(commands)).then(|| ()).ok_or(())
            }
            Commands::GenReadme => {
                dev_tool::generate_readme();
                Ok(())
            }
            Commands::ServeDemo => map_exit_status(
                Command::new("trunk")
                    .args(["serve", "example/index.html"])
                    .run(),
            ),
            Commands::BuildDocs => map_exit_status(
                Command::new("cargo")
                    .args([
                        "+nightly",
                        "doc",
                        "-p",
                        "yew-interop",
                        "--features",
                        "yew-stable",
                        "--features",
                        "script",
                        "--open",
                    ])
                    .run_with_note(
                        "Note: CI will build the docs without the dependencies to save space,\
                \nso the links to other crates will appear unresolved.\n\
                 This won't be a problem when the docs are published.",
                    ),
            ),
        }
    }
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    if let Some(subcommand) = args.command.or_else(|| {
        let commands: Vec<_> = Commands::iter().collect();
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Type or use the arrow key to select the task")
            .items(&commands)
            .interact_opt()
            .unwrap();
        selection.map(|s| commands[s])
    }) {
        subcommand.run()
    } else {
        Ok(())
    }
}
