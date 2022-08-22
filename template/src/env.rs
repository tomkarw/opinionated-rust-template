use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Env {
    example_boolean: bool,
    example_list: Vec<String>,
}

/// Access to parsed environment variables.
pub static ENV: Lazy<Env> = Lazy::new(|| envy::from_env().unwrap());
