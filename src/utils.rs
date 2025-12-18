use crate::errors::AppError;
use semver::Version;
use std::fs;
use std::path::Path;

pub fn get_package_version<P: AsRef<Path>>(directory: P) -> Result<Version, AppError> {
    let cargo_toml_path = directory.as_ref().join("Cargo.toml");

    // Read the Cargo.toml file
    let contents = fs::read_to_string(&cargo_toml_path)?;

    // Parse the TOML
    let toml: toml::Value =
        toml::from_str(&contents).map_err(|err| AppError::ParseError(err.to_string()))?;

    // Extract version from [package] section
    let version_str = toml
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ParseError("Version not found in Cargo.toml".to_string()))?;

    let package_version = semver::Version::parse(version_str)?;

    Ok(package_version)
}
