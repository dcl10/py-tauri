use core::str;
use semver::{Version, VersionReq};

pub const MINIMUM_PYTHON: &str = ">=3.10";
pub const PYTHON_SUFFIXES: [&str; 2] = ["", "3"];

pub fn check_python_version(python_version: &str) -> bool {
    let minimum_py = VersionReq::parse(MINIMUM_PYTHON).unwrap();
    if let Ok(actual_version) = Version::parse(python_version.trim()) {
        return minimum_py.matches(&actual_version);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::check_python_version;

    #[test]
    fn test_check_python_version() {
        // Arrange
        let good_version = "3.11.9";
        let bad_version = "2.7.13";

        // Act
        let is_good = check_python_version(good_version);
        let is_bad = !check_python_version(bad_version);

        // Assert
        assert!(is_good);
        assert!(is_bad);
    }
}
