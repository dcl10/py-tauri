use core::str;
use std::{process::Command, str::FromStr};
use semver::{Version, VersionReq};
use tauri::AppHandle;

pub const MINIMUM_PYTHON: &str = ">=3.10";
pub const PYTHON_SUFFIXES: [&str; 2] = ["", "3"];

pub fn check_python_version(python_version: &str) -> bool {
    let minimum_py = VersionReq::parse(MINIMUM_PYTHON).unwrap();
    if let Ok(actual_version) = Version::parse(python_version.trim()) {
        return minimum_py.matches(&actual_version);
    }
    false
}

pub fn create_venv(path: &str, exe: &str, app_handle: AppHandle) -> Result<String, String> {
    let app_dir = app_handle.path_resolver().app_data_dir();
    if app_dir.is_none() {
        return Err("Could not locate app data directory".to_string());
    }
    let venv_dir_buf = app_dir.unwrap().join(path);
    let venv_dir_str_opt = venv_dir_buf.to_str();
    if venv_dir_str_opt.is_none() {
        return Err("Invalid virtual environment path".to_string());
    }
    let venv_str = venv_dir_str_opt.unwrap();
    let cmd = Command::new(exe)
        .args(&["-m", "venv", venv_str])
        .status();

    match cmd {
        Ok(_) => {
            let python_path_buf = venv_dir_buf.join("bin/python");
            let python_path_str_opt = python_path_buf.to_str();
            let python = String::from_str(python_path_str_opt.unwrap());
            Ok(python.unwrap())
        },
        Err(_) => Err("Could not create the python virtual environment".to_string())
    }
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
