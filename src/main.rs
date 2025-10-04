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
        for (pat, to) in self.iter() {
            for from in files
                .iter()
                .filter(|f| f.path().is_file() && pat.is_match(&f.path().display().to_string()))
            {
                let from = from.path();
                let name = from
                    .file_name()
                    .expect("no file name even though we verified is file");
                fs::create_dir_all(to)?;
                let to = to.join(name);
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
