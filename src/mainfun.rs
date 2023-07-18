// #![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(required=true)]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    println!("file content: {}", content);
    Ok(())
}
