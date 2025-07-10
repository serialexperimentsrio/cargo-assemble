use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;
use std::env;

mod cmd;
mod ws;

use cmd::Args;

fn main() -> Result<()> {
    // Handle cargo subcommand: filter out "assemble" argument
    let args: Vec<String> = env::args().collect();
    let filtered_args: Vec<String> = if args.len() > 1 && args[1] == "assemble" {
        let mut new_args = args;
        new_args.remove(1);
        new_args
    } else {
        args
    };

    let args = Args::parse_from(filtered_args);

    // Convert argument `Vec<String>` into hash sets for quick lookups
    let lib_members: HashSet<_> = args.lib.iter().cloned().collect();
    let bin_members: HashSet<_> = args.bin.iter().cloned().collect();

    // Track whether flag is used
    let lib_flag_used = !lib_members.is_empty();
    let bin_flag_used = !bin_members.is_empty();

    let cargo_toml = ws::read_cargo_toml()?;

    for member in cargo_toml.workspace.members {
        // Skip existing member
        if ws::member_exists(&member) {
            ws::print_skip_message(&member);
            continue;
        }

        // Determine whether member should be lib or bin
        let is_lib = if lib_flag_used {
            lib_members.contains(&member)
        } else if bin_flag_used {
            !bin_members.contains(&member)
        } else {
            // Default to binary
            false
        };

        cmd::create_member(&member, is_lib)?;
    }

    Ok(())
}
