use crate::types::BumpType;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Script to create and push a new tag for a Rust repository."
)]
pub struct App {
    #[arg(long, help = "Run all checks but do not push tag.")]
    pub dry_run: bool,

    #[arg(long, help = "Path to directory with manifest files (Cargo.toml)")]
    pub directory: PathBuf,

    #[arg(long, help = "Bump type")]
    pub bump_type: BumpType,
}
