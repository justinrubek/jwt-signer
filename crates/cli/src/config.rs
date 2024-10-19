use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    pub secret_key: String,
    pub issuer: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("JWT_SIGNER"))
            .build()?;

        config.try_deserialize()
    }
}
