use config::{ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub eventstore_url: String,
    pub redis_url: String,
    pub redis_channel: String,
    pub couchbase_url: String,
    pub couchbase_username: String,
    pub couchbase_password: String,
    pub couchbase_bucket: String,
    pub sentry_dsn: String,
}

impl AppConfig {
    fn create() -> Result<Self, ConfigError> {
        let mut config = config::Config::new();
        config.merge(Environment::with_prefix("cobase_worker"))?;

        config.try_into()
    }
}

lazy_static::lazy_static! {
    pub static ref APP_CONFIG: AppConfig = {
        dotenv::from_filename(".env.local").ok();
        dotenv::dotenv().ok();

        AppConfig::create().unwrap_or_else(|e| panic!(e))
    };
}
