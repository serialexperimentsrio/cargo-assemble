use anyhow::{Context, Result};
use colored::*;

use serde::Deserialize;

use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,
}

// Read and parse workspace member list from the root `Cargo.toml`
pub fn read_cargo_toml() -> Result<CargoToml> {
    let cargo_toml_content =
        fs::read_to_string("Cargo.toml").context("Failed to read `Cargo.toml` file")?;

    let cargo_toml =
        toml::from_str(&cargo_toml_content).context("Failed to parse `Cargo.toml` file")?;

    Ok(cargo_toml)
}

// Check if member dir exists already
pub fn member_exists(member: &str) -> bool {
    Path::new(member).exists()
}

pub fn print_skip_message(member: &str) {
    println!(
        "{} `{}`, already exists",
        "Skipping".yellow().bold(),
        member
    );
}
