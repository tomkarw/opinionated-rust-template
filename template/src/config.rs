/*!
Provide a crate wide configuration singleton.
As of now, data is sources from environment variables.
!*/

use once_cell::sync::Lazy;
use serde::Deserialize;

/// Configuration variables for the crate.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// `EXAMPLE_BOOL`
    ///
    /// An example of simple value sourced from environment variables.
    pub example_bool: bool,
    /// `EXAMPLE_LISt`
    ///
    /// An example of advanced value sourced from environment variables.
    pub example_list: Vec<String>,
}

/// Access to parsed configuration.
pub static CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().expect("some env vars missing"));

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::Config;

    #[test]
    fn test_create_config() {
        let config = Config {
            example_bool: true,
            example_list: vec!["one".to_owned(), "two".to_owned(), "three".to_owned()],
        };
        assert_eq!(config.example_bool, true);
        assert_eq!(
            config.example_list,
            vec!["one".to_owned(), "two".to_owned(), "three".to_owned()]
        );
    }
}
