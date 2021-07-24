mod app_config;
mod group;
mod response;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Condition, App, HttpServer};
use app_config::APP_CONFIG;
use cobase_core::Bus;
use couchbase::Cluster;
use sentry::{configure_scope, ClientOptions};
use std::sync::Arc;

#[macro_use]
extern crate log;

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

    let couchbase_cluster = Cluster::connect(
        app_config.couchbase_url,
        app_config.couchbase_username,
        app_config.couchbase_password,
    );

    let couchbase_bucket =
        Arc::new(couchbase_cluster.bucket(app_config.couchbase_bucket.to_owned()));
    let couchbase_cluster = Arc::new(couchbase_cluster);

    let bus = Bus {
        eventstore_client,
        couchbase_cluster,
        couchbase_bucket,
    }
    .start();

    let cors = app_config.cors.unwrap_or(false);

    HttpServer::new(move || {
        App::new()
            //.wrap(sentry_actix::Sentry::new())
            .wrap(Condition::new(cors, Cors::permissive()))
            .data(bus.clone())
            .configure(group::configure)
    })
    .bind(("0.0.0.0", app_config.port))?
    .run()
    .await
}
