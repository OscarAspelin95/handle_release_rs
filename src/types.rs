use clap::ValueEnum;
use rstest::rstest;
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

#[rstest]
#[case(BumpType::Patch, Version::new(0, 0, 0), Version::new(0, 0, 1))]
#[case(BumpType::Minor, Version::new(0, 0, 1), Version::new(0, 1, 0))]
#[case(BumpType::Minor, Version::new(0, 1, 0), Version::new(0, 2, 0))]
#[case(BumpType::Major, Version::new(0, 2, 0), Version::new(1, 0, 0))]
#[case(BumpType::Major, Version::new(1, 0, 0), Version::new(2, 0, 0))]
fn test_bump_version(
    #[case] bump_type: BumpType,
    #[case] version: Version,
    #[case] expected: Version,
) {
    let result = bump_type.bump_version(version);
    assert_eq!(result, expected);
}
