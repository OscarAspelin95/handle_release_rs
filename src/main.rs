use clap::Parser;
use simple_logger::SimpleLogger;

mod types;

mod errors;
use errors::AppError;

mod args;
use args::App;

mod git;
use git::{ensure_main_branch, ensure_no_dirty, get_latest_tag_on_main, make_and_push_tag};

mod utils;
use utils::get_package_version;

fn main() -> Result<(), AppError> {
    SimpleLogger::new().init()?;

    let args = App::parse();

    let package_version = get_package_version(&args.directory)?;

    ensure_main_branch()?;
    ensure_no_dirty()?;

    let current_git_version = get_latest_tag_on_main()?;

    let bump_type = args.bump_type;
    let new_git_version = bump_type.bump_version(current_git_version.clone());

    if package_version != new_git_version {
        return Err(AppError::VersionMismatchError(format!(
            "New git tag {new_git_version} does not match current package version {package_version}. \
            Make sure to update and commit the Cargo.toml and Cargo.lock versions."
        )));
    }

    if !args.dry_run {
        make_and_push_tag(current_git_version, new_git_version)?;
    }

    Ok(())
}
