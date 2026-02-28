use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Settings;

impl Settings {
    pub fn from_env() -> Result<Self, SettingsError> {
        Ok(Self::default())
    }

    pub fn get_value(&self, key: &str) -> Result<String, SettingsError> {
        match env::var(key) {
            Ok(value) if !value.is_empty() => Ok(value),
            _ => Err(SettingsError::MissingEnvVar(key.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsError {
    MissingEnvVar(String),
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::MissingEnvVar(key) => {
                write!(f, "Environment variable {key} is required")
            }
        }
    }
}

impl Error for SettingsError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_env_var<F: FnOnce()>(key: &str, value: Option<&str>, test: F) {
        let original = env::var(key).ok();
        match value {
            Some(v) => env::set_var(key, v),
            None => env::remove_var(key),
        }
        test();
        match original {
            Some(v) => env::set_var(key, v),
            None => env::remove_var(key),
        }
    }

    #[test]
    fn returns_value_when_present() {
        with_env_var("TEST_KEY", Some("abc123"), || {
            let settings = Settings::from_env().expect("should init settings");
            let value = settings.get_value("TEST_KEY").expect("should load value");
            assert_eq!(value, "abc123");
        });
    }

    #[test]
    fn errors_when_missing_value() {
        with_env_var("TEST_KEY", None, || {
            let settings = Settings::from_env().expect("should init settings");
            let result = settings.get_value("TEST_KEY");
            assert!(matches!(result, Err(SettingsError::MissingEnvVar(key)) if key == "TEST_KEY"));
        });
    }
}
