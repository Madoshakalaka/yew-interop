use clap::Parser;
use dev_tool::*;
use semver::{Error as SemverError, Version as SemverVersion};
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Debug)]
enum Version {
    Master,
    Semver(String),
}

impl Version {
    fn to_sub_folder(&self) -> Cow<'static, str> {
        match self {
            Version::Master => "master".into(),
            Version::Semver(v) => format!("v{}", v).into(),
        }
    }
}

impl FromStr for Version {
    type Err = SemverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "master" | "next" => Ok(Self::Master),
            other => SemverVersion::parse(other.strip_prefix('v').unwrap_or(other))
                .map(|v| Version::Semver(v.to_string())),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(name = "build-demo")]
#[clap(about = "build the demo website so that it can be deployed on github pages")]
struct Args {
    /// master, or a full semver string like 2.1.0
    #[clap(parse(try_from_str))]
    version: Version,
}

fn main() -> ! {
    let args = Args::parse();
    let sub_folder: Cow<'static, str> = args.version.to_sub_folder();
    Command::new("trunk")
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
        .run();
}
