use clap::ValueEnum;
use semver::Version;

#[derive(Debug, Clone, ValueEnum)]
pub enum BumpType {
    Major,
    Minor,
    Patch,
}

impl BumpType {
    pub fn bump_version(&self, mut version: Version) -> Version {
        match self {
            BumpType::Major => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0;
            }
            BumpType::Minor => {
                version.minor += 1;
                version.patch = 0;
            }
            BumpType::Patch => {
                version.patch += 1;
            }
        }

        version
    }
}
