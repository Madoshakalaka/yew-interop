use clap::Parser;
use semver::{Error as SemverError, Version as SemverVersion};
use std::borrow::Cow;
use std::process::{Command, Stdio};
use std::str::FromStr;

#[derive(Debug)]
enum Version {
    Next,
    SemverVersion(String),
}
impl FromStr for Version {
    type Err = SemverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "next" | "master" => Ok(Self::Next),
            other => SemverVersion::parse(other).map(|_| Version::SemverVersion(s.to_string())),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(name = "build-demo")]
#[clap(about = "build the demo website so that it can be deployed on github pages")]
struct Args {
    /// next, or a full semver string like 2.1.0
    #[clap(parse(try_from_str), default_value = "next")]
    version: Version,
}

fn main() {
    let args = Args::parse();
    let sub_folder: Cow<'static, str> = match args.version {
        Version::Next => "next".into(),
        Version::SemverVersion(v) => v.into(),
    };
    let status = Command::new("trunk")
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
