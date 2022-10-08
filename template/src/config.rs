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
    /// An example of simple value sourced from env vars.
    pub example_bool: bool,
    /// `EXAMPLE_LISt`
    ///
    /// An example of advanced value sourced from env vars.
    pub example_list: Vec<String>,
}

/// Access to parsed configuration.
pub static CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().expect("some env vars missing"));
