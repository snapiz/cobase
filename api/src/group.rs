use actix::Addr;
use actix_web::{
    get, post,
    web::{self, ReqData},
    HttpResponse, Result,
};
use actix_web_middleware_keycloak_auth::{Claims, KeycloakAuth};
pub use cobase_core::group::{command::CreateGroupCommand, query::FindGroupsQueryMessage};
pub use cobase_core::{Bus, Message};

use crate::app_config;
use crate::response::{CommandResponse, Response};

#[get("")]
async fn index(bus: web::Data<Addr<Bus>>, claims: ReqData<Claims>) -> Result<HttpResponse> {
    let res = bus
        .send(FindGroupsQueryMessage(claims.sub.to_string()))
        .await;

    Ok(Response(res).into())
}

#[post("/create")]
async fn create(
    bus: web::Data<Addr<Bus>>,
    claims: ReqData<Claims>,
    input: web::Json<CreateGroupCommand>,
) -> Result<HttpResponse> {
    let res = bus
        .send(Message(claims.sub.to_string(), input.0))
        .await;

    Ok(CommandResponse(res).into())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let keycloak_auth = KeycloakAuth {
        detailed_responses: true,
        required_roles: vec![],
        keycloak_oid_public_key: app_config::AppConfig::keycloak_oid_public_key(),
    };

    cfg.service(
        web::scope("/groups")
            .wrap(keycloak_auth)
            .service(index)
            .service(create),
    );
}
