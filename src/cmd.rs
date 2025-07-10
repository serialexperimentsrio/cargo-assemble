use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use colored::*;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .args(&["lib", "bin"])
        .multiple(false)
))]
pub struct Args {
    /// Create member as library project
    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    pub lib: Vec<String>,

    /// Create member as binary project
    #[arg(long, num_args = 1.., value_delimiter = ' ')]
    pub bin: Vec<String>,
}

pub fn create_member(member: &str, is_lib: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("new").arg(member);
    if is_lib {
        cmd.arg("--lib");
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute 'cargo new' for member '{}'", member))?;

    if output.status.success() {
        println!(
            "{} {} `{}` package",
            "Creating".green().bold(),
            if is_lib {
                "library"
            } else {
                "binary (application)"
            },
            member
        );
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to create `{}`:\n{}",
            member,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
