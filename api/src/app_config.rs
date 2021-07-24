use actix_web_middleware_keycloak_auth::DecodingKey;
use config::{ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub cors: Option<bool>,
    pub keycloak_pk: String,
    pub eventstore_url: String,
    pub couchbase_url: String,
    pub couchbase_username: String,
    pub couchbase_password: String,
    pub couchbase_bucket: String,
    pub sentry_dsn: String,
}

impl AppConfig {
    fn create() -> Result<Self, ConfigError> {
        let mut config = config::Config::new();
        config.merge(Environment::with_prefix("cobase_api"))?;

        config.try_into()
    }

    pub fn keycloak_oid_public_key() -> DecodingKey<'static> {
        DecodingKey::from_rsa_pem(KEYCLOAK_PK.as_bytes()).unwrap_or_else(|e| panic!(e))
    }
}

lazy_static::lazy_static! {
    pub static ref APP_CONFIG: AppConfig = {
        dotenv::from_filename(".env.local").ok();
        dotenv::dotenv().ok();

        AppConfig::create().unwrap_or_else(|e| panic!(e))
    };

    pub static ref KEYCLOAK_PK: String = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", APP_CONFIG.keycloak_pk);
}
