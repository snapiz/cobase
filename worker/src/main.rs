mod app_config;

use actix::Actor;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer};
use app_config::APP_CONFIG;
use cobase_core::{group, Publisher};
use couchbase::Cluster;
use mobc_redis::redis;
use sentry::{ClientOptions, configure_scope};
use std::sync::Arc;

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::NoContent().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.local").ok();
    dotenv::dotenv().ok();

    let logger = sentry_log::SentryLogger::with_dest(env_logger::builder().build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let app_config = APP_CONFIG.clone();

    let _guard = sentry::init((
        app_config.sentry_dsn.to_owned(),
        ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    configure_scope(|scope| {
        scope.set_tag("application", env!("CARGO_PKG_NAME"));
    });

    let evenstore_settings = app_config
        .eventstore_url
        .parse()
        .expect("Failed to parse eventstore settings");

    let eventstore_client: eventstore::Client = eventstore::Client::create(evenstore_settings)
        .await
        .expect("Failed to create eventstore client");

    let redis_client =
        redis::Client::open(app_config.redis_url).expect("Failed to create redis client");

    let couchbase_cluster = Cluster::connect(
        app_config.couchbase_url,
        app_config.couchbase_username,
        app_config.couchbase_password,
    );

    let couchbase_bucket =
        Arc::new(couchbase_cluster.bucket(app_config.couchbase_bucket.to_owned()));
    let couchbase_cluster = Arc::new(couchbase_cluster);
    let publisher = Publisher::new(app_config.redis_channel, redis_client).start();

    group::query::EventHandler {
        eventstore_client: eventstore_client.clone(),
        couchbase_cluster: couchbase_cluster.clone(),
        couchbase_bucket: couchbase_bucket.clone(),
        publisher,
    }
    .start();

    HttpServer::new(move || App::new().wrap(Logger::default()).service(health))
        .bind(("0.0.0.0", app_config.port))?
        .run()
        .await
}
