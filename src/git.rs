use crate::errors::AppError;
use log::{error, info};
use semver::Version;
use std::process::{Command, exit};

use std::io::stdin;

pub fn ensure_main_branch() -> Result<(), AppError> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()?;

    let branch = String::from_utf8(output.stdout)?;
    match branch.trim() {
        "main" => Ok(()),
        _ => Err(AppError::NotOnMainBranchError(branch)),
    }
}

pub fn ensure_no_dirty() -> Result<(), AppError> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()?;

    let result = String::from_utf8(output.stdout)?;
    match result.trim() {
        "" => Ok(()),
        _ => Err(AppError::DirtyWorkTreeError(result)),
    }
}

pub fn get_latest_tag_on_main() -> Result<Version, AppError> {
    let output = Command::new("git")
        .arg("describe")
        .arg("--tags")
        .arg("--abbrev=0")
        .arg("origin/main")
        .output()?;

    let latest_tag = String::from_utf8(output.stdout)?;

    if !latest_tag.starts_with("v") {
        error!("Found invalig tag {}", latest_tag);
        return Err(AppError::InvalidTagError(latest_tag));
    }

    let latest_version = semver::Version::parse(latest_tag[1..].trim())?;

    Ok(latest_version)
}

pub fn make_and_push_tag(current_version: Version, new_version: Version) -> Result<(), AppError> {
    let new_tag = format!("v{}", new_version);

    info!("Bump v{current_version} -> {new_tag}? (y/n): ");
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match input.as_ref() {
        "y" => {}
        _ => exit(1),
    }

    let _ = Command::new("git")
        .arg("tag")
        .arg("-am")
        .arg(&new_tag)
        .arg(&new_tag)
        .output()?;

    let _ = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(&new_tag)
        .output()?;

    Ok(())
}
