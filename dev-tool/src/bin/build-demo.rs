use clap::Parser;
use derive_more::Display;
use semver::{Error as SemverError, Version as SemverVersion};
use std::borrow::Cow;
use std::process::{Command, Stdio};
use std::str::FromStr;

#[derive(Debug, Display)]
enum Version {
    YewNext,
    Master,
    Semver(String),
}

impl Version {
    fn into_sub_folder(self) -> Cow<'static, str> {
        match self {
            Version::YewNext => "yew-next".into(),
            Version::Master => "master".into(),
            Version::Semver(v) => v.into(),
        }
    }
}
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

impl TryFrom<Vec<u8>> for Version {
    type Error = <Version as FromStr>::Err;

    fn try_from(utf_8: Vec<u8>) -> Result<Self, Self::Error> {
        let mut s = String::from_utf8(utf_8).unwrap();
        remove_whitespace(&mut s);
        let s = s.strip_prefix('v').unwrap_or(&s);
        Self::from_str(s)
    }
}

impl FromStr for Version {
    type Err = SemverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yew-next" | "next" => Ok(Self::YewNext),
            "master" => Ok(Self::Master),
            other => SemverVersion::parse(other).map(|_| Version::Semver(s.to_string())),
        }
    }
}

fn from_branch_name() -> Version {
    let command = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();
    assert!(
        command.status.success(),
        "failed to get the current branch name, do you want to manually specify the demo version?"
    );
    let version = command
        .stdout
        .try_into()
        .expect("failed to convert branch name into demo version");

    println!("Successfully parsed version: {}", version);

    version
}

#[derive(Parser, Debug)]
#[clap(name = "build-demo")]
#[clap(about = "build the demo website so that it can be deployed on github pages")]
struct Args {
    /// yew-next, master, or a full semver string like 2.1.0
    /// if omitted, will guess from the current git branch name
    #[clap(parse(try_from_str))]
    version: Option<Version>,
}

fn main() {
    let args = Args::parse();
    let sub_folder: Cow<'static, str> = args
        .version
        .unwrap_or_else(|| {
            println!("version argument is omitted, trying to guess from the current branch name");
            from_branch_name()
        })
        .into_sub_folder();
    let status = Command::new("trunk")
        .env("YEW_INTEROP_DEMO_VERSION", &*sub_folder)
        .args([
            "build",
            "--release",
            "--dist",
            &format!("docs/{}", sub_folder),
            "--public-url",
            &format!("/yew-interop/{}", sub_folder),
            "example/index.html",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap()
        .status;
    std::process::exit(status.code().unwrap())
}
