use clap::Parser;
use log::info;

use simple_logger::SimpleLogger;

mod types;

mod errors;
use errors::AppError;

mod args;
use args::App;

mod git;
use git::{ensure_main_branch, get_latest_tag_on_main, make_and_push_tag};

mod utils;
use utils::get_package_version;

fn main() -> Result<(), AppError> {
    SimpleLogger::new().init()?;

    let args = App::parse();

    info!("Getting cargo package version...");
    let package_version = get_package_version(&args.directory)?;

    info!("Ensuring main branch...");
    ensure_main_branch()?;

    info!("Fetching latest tag on main...");
    let current_git_version = get_latest_tag_on_main()?;

    info!("Determining new tag...");
    let bump_type = args.bump_type;
    let new_git_version = bump_type.bump_version(current_git_version.clone());

    if package_version != new_git_version {
        return Err(AppError::VersionMismatchError(format!(
            r#"
            New git tag `{new_git_version}` does not match current package version `{package_version}`.

            Make sure Cargo.toml and Cargo.lock has been updated
            to the latest version {new_git_version}. Remember to commit.
            "#,
        )));
    }

    if !args.dry_run {
        make_and_push_tag(current_git_version, new_git_version)?;
    }

    Ok(())
}
