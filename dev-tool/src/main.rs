use clap::{Parser, Subcommand};
use derive_more::Display;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Parser)]
#[clap(about = "meant to be used locally by developers")]
struct Args {
    /// omit this command for an interactive menu
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, EnumIter, Display, Copy, Clone)]
enum Commands {
    /// run all checks
    PrePr,
    /// generate README.md from yew-interop/src/docs.md
    GenReadme,
    /// serve the demo website under ./example
    ServeDemo,
}

impl Commands {
    fn run(&self) {
        match self {
            Commands::PrePr => {
                println!("pre pr")
            }
            Commands::GenReadme => {
                let mut output = String::new();

                let content = include_str!("../../yew-interop/src/docs.md");

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

                fs::write("README.md", output).unwrap();
            }
            Commands::ServeDemo => {
                println!("serve!")
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    if let Some(subcommand) = args.command.or_else(|| {
        let commands: Vec<_> = Commands::iter().collect();
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("which command do you want to run?")
            .items(&commands)
            .interact_opt()
            .unwrap();
        selection.map(|s| commands[s])
    }) {
        subcommand.run()
    }
}
