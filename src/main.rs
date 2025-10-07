#![warn(clippy::pedantic)]
use anyhow::{Context, Result, bail};
use clap::Parser;
use regex::Regex;
use std::fs::DirEntry;
use std::ops::Deref;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    path: Option<PathBuf>,
}

struct SortRules(Vec<(Regex, PathBuf)>);

impl SortRules {
    fn sort(&self) -> Result<()> {
        let files: Vec<DirEntry> =
            fs::read_dir(".")?.collect::<std::result::Result<Vec<_>, _>>()?;
        for f in &files {
            let from = f.path();
            let name = f.file_name();
            if from.is_file()
                && let Some((_, to)) = self.iter().find(|(pat, _)| {
                    pat.is_match(name.to_str().expect("non-unicode data in file name"))
                })
            {
                fs::create_dir_all(to)?;
                let to = to.join(&name);
                println!("Moving: {} -> {}", name.display(), to.display());
                fs::rename(from, to)?;
            }
        }
        Ok(())
    }
}

impl TryFrom<String> for SortRules {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let mut out = Vec::new();
        for l in value.lines() {
            let Some((k, v)) = l.split_once("->") else {
                bail!("malformed sort file at: {l}");
            };
            out.push((Regex::new(k.trim())?, PathBuf::from(v.trim())));
        }
        Ok(SortRules(out))
    }
}

impl Deref for SortRules {
    type Target = Vec<(Regex, PathBuf)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(path) = args.path {
        env::set_current_dir(path)?;
    }
    let sort_rules: SortRules = fs::read_to_string(".sort")
        .context(".sort file does not exist in directory")?
        .try_into()?;
    sort_rules.sort()
}
