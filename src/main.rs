#![warn(clippy::pedantic)]
#![feature(iterator_try_collect)]
use anyhow::{Result, bail};
use clap::Parser;
use std::fs::DirEntry;
use std::ops::Deref;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    path: Option<PathBuf>,
}

struct SortRules(Vec<(String, String)>);

impl SortRules {
    fn sort(&self) -> Result<()> {
        let files: Vec<DirEntry> = fs::read_dir(".")?.try_collect()?;
        for (pat, to) in self.iter() {
            for from in files
                .iter()
                .filter(|f| f.path().is_file() && f.path().display().to_string().contains(pat))
            {
                let from = from.path();
                let name = from
                    .file_name()
                    .expect("no file name even though we verified is file");
                fs::create_dir_all(to)?;
                let to = PathBuf::from(to).join(name);
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
            out.push((k.trim().to_string(), v.trim().to_string()));
        }
        Ok(SortRules(out))
    }
}

impl Deref for SortRules {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(path) = args.path {
        env::set_current_dir(path)?;
    }
    let sort_rules: SortRules = fs::read_to_string(".sort")?.try_into()?;
    sort_rules.sort()
}
