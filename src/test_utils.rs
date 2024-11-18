use crate::Config;
use anyhow::Result;

#[cfg(test)]
pub fn print_assert_failed(title: &str, expected: &str, got: &str) -> String {
    format!(
        "{} failed: Expected '{:?}', but Got '{:?}'",
        title, expected, got
    )
}

#[cfg(test)]
pub fn get_test_config() -> Result<Config> {
    Config::new()
}
